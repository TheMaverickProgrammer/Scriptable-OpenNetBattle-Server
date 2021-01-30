use super::super::{build_packet, ClientPacket, PacketHeaders, ServerPacket};
use super::{get_reliability_byte, Reliability};
use std::net::UdpSocket;

struct BackedUpPacket {
  pub id: u64,
  pub packet: ClientPacket,
}

pub struct PacketSorter {
  socket_address: std::net::SocketAddr,
  next_reliable: u64,
  next_unreliable_sequenced: u64,
  next_reliable_ordered: u64,
  missing_reliable: Vec<u64>,
  backed_up_ordered_packets: Vec<BackedUpPacket>,
}

impl PacketSorter {
  pub fn new(socket_address: std::net::SocketAddr) -> PacketSorter {
    PacketSorter {
      socket_address,
      next_reliable: 0,
      next_unreliable_sequenced: 0,
      next_reliable_ordered: 0,
      missing_reliable: Vec::new(),
      backed_up_ordered_packets: Vec::new(),
    }
  }

  pub fn sort_packet(
    &mut self,
    socket: &UdpSocket,
    headers: PacketHeaders,
    packet: ClientPacket,
  ) -> std::io::Result<Vec<ClientPacket>> {
    let packets = match headers.reliability {
      Reliability::Unreliable => vec![packet],
      Reliability::UnreliableSequenced => {
        if headers.id < self.next_unreliable_sequenced {
          // ignore old packets
          vec![]
        } else {
          self.next_unreliable_sequenced = headers.id + 1;
          vec![packet]
        }
      }
      Reliability::Reliable => {
        self.send_ack(socket, &headers)?;

        if headers.id == self.next_reliable {
          // expected
          self.next_reliable += 1;
          vec![packet]
        } else if headers.id > self.next_reliable {
          // skipped expected
          self.missing_reliable.extend(self.next_reliable..headers.id);
          self.next_reliable = headers.id + 1;

          vec![packet]
        } else if let Some(i) = self
          .missing_reliable
          .iter()
          .position(|id| *id == headers.id)
        {
          // one of the missing packets
          self.missing_reliable.remove(i);

          vec![packet]
        } else {
          // we already handled this packet
          vec![]
        }
      }
      Reliability::ReliableOrdered => {
        self.send_ack(socket, &headers)?;

        if headers.id == self.next_reliable_ordered {
          let mut i = 0;

          for backed_up_packet in &self.backed_up_ordered_packets {
            self.next_reliable_ordered += 1;

            if backed_up_packet.id != self.next_reliable_ordered {
              break;
            }

            i += 1;
          }

          // split backed up packets, store newer packets
          let mut backed_up = self.backed_up_ordered_packets.split_off(i);
          // swap newer packets for older packets
          std::mem::swap(&mut self.backed_up_ordered_packets, &mut backed_up);

          let mut packets = vec![packet];
          packets.extend(backed_up.into_iter().map(|bp| bp.packet));
          packets
        } else if headers.id > self.next_reliable {
          // sorted insert
          let mut i = 0;
          let mut should_insert = true;

          for backed_up_packet in &self.backed_up_ordered_packets {
            if backed_up_packet.id == headers.id {
              should_insert = false;
              break;
            }
            if backed_up_packet.id > headers.id {
              break;
            }
            i += 1;
          }

          if should_insert {
            self.backed_up_ordered_packets.insert(
              i,
              BackedUpPacket {
                id: headers.id,
                packet,
              },
            );
          }

          vec![]
        } else {
          // already handled
          vec![]
        }
      }
    };

    Ok(packets)
  }

  fn send_ack(&self, socket: &UdpSocket, headers: &PacketHeaders) -> std::io::Result<()> {
    let mut buf = vec![0];

    buf.extend(build_packet(&ServerPacket::Ack {
      reliability: get_reliability_byte(&headers.reliability),
      id: headers.id,
    }));

    socket.send_to(&buf, self.socket_address)?;

    Ok(())
  }
}