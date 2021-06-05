use super::lua_errors::{create_area_error, create_player_error};
use super::LuaApi;
use crate::net::Direction;

#[allow(clippy::type_complexity)]
pub fn inject_dynamic(lua_api: &mut LuaApi) {
  lua_api.add_dynamic_function("Net", "list_players", |api_ctx, lua_ctx, params| {
    let area_id: rlua::String = lua_ctx.unpack_multi(params)?;
    let area_id_str = area_id.to_str()?;

    let mut net = api_ctx.net_ref.borrow_mut();

    if let Some(area) = net.get_area_mut(area_id_str) {
      let connected_players_iter = area.get_connected_players().iter();
      let result: rlua::Result<Vec<rlua::String>> = connected_players_iter
        .map(|player_id| lua_ctx.create_string(player_id))
        .collect();

      lua_ctx.pack_multi(result?)
    } else {
      Err(create_area_error(area_id_str))
    }
  });

  lua_api.add_dynamic_function("Net", "is_player", |api_ctx, lua_ctx, params| {
    let player_id: rlua::String = lua_ctx.unpack_multi(params)?;
    let player_id_str = player_id.to_str()?;

    let net = api_ctx.net_ref.borrow();

    let player_exists = net.get_player(player_id_str).is_some();

    lua_ctx.pack_multi(player_exists)
  });

  lua_api.add_dynamic_function("Net", "get_player_area", |api_ctx, lua_ctx, params| {
    let player_id: rlua::String = lua_ctx.unpack_multi(params)?;
    let player_id_str = player_id.to_str()?;

    let net = api_ctx.net_ref.borrow_mut();

    if let Some(player) = net.get_player(player_id_str) {
      lua_ctx.pack_multi(player.area_id.as_str())
    } else {
      Err(create_player_error(player_id_str))
    }
  });

  lua_api.add_dynamic_function("Net", "get_player_ip", |api_ctx, lua_ctx, params| {
    let player_id: rlua::String = lua_ctx.unpack_multi(params)?;
    let player_id_str = player_id.to_str()?;

    let net = api_ctx.net_ref.borrow();

    if let Some(addr) = net.get_player_addr(player_id_str) {
      lua_ctx.pack_multi(addr.ip().to_string())
    } else {
      Err(create_player_error(player_id_str))
    }
  });

  lua_api.add_dynamic_function("Net", "get_player_name", |api_ctx, lua_ctx, params| {
    let player_id: rlua::String = lua_ctx.unpack_multi(params)?;
    let player_id_str = player_id.to_str()?;

    let net = api_ctx.net_ref.borrow_mut();

    if let Some(player) = net.get_player(player_id_str) {
      lua_ctx.pack_multi(player.name.as_str())
    } else {
      Err(create_player_error(player_id_str))
    }
  });

  lua_api.add_dynamic_function("Net", "set_player_name", |api_ctx, lua_ctx, params| {
    let (player_id, name): (rlua::String, String) = lua_ctx.unpack_multi(params)?;
    let player_id_str = player_id.to_str()?;

    let mut net = api_ctx.net_ref.borrow_mut();

    net.set_player_name(player_id_str, name);

    lua_ctx.pack_multi(())
  });

  lua_api.add_dynamic_function("Net", "get_player_direction", |api_ctx, lua_ctx, params| {
    let player_id: rlua::String = lua_ctx.unpack_multi(params)?;
    let player_id_str = player_id.to_str()?;

    let net = api_ctx.net_ref.borrow();

    if let Some(player) = net.get_player(player_id_str) {
      let direction_str = player.direction.as_str();

      lua_ctx.pack_multi(direction_str)
    } else {
      Err(create_player_error(player_id_str))
    }
  });

  lua_api.add_dynamic_function("Net", "get_player_position", |api_ctx, lua_ctx, params| {
    let player_id: rlua::String = lua_ctx.unpack_multi(params)?;
    let player_id_str = player_id.to_str()?;

    let net = api_ctx.net_ref.borrow();

    if let Some(player) = net.get_player(player_id_str) {
      let table = lua_ctx.create_table()?;
      table.set("x", player.x)?;
      table.set("y", player.y)?;
      table.set("z", player.z)?;

      lua_ctx.pack_multi(table)
    } else {
      Err(create_player_error(player_id_str))
    }
  });

  lua_api.add_dynamic_function("Net", "get_player_mugshot", |api_ctx, lua_ctx, params| {
    let player_id: rlua::String = lua_ctx.unpack_multi(params)?;
    let player_id_str = player_id.to_str()?;

    let net = api_ctx.net_ref.borrow();

    if let Some(player) = net.get_player(player_id_str) {
      let table = lua_ctx.create_table()?;
      table.set("texture_path", player.mugshot_texture_path.as_str())?;
      table.set("animation_path", player.mugshot_animation_path.as_str())?;

      lua_ctx.pack_multi(table)
    } else {
      Err(create_player_error(player_id_str))
    }
  });

  lua_api.add_dynamic_function("Net", "get_player_avatar", |api_ctx, lua_ctx, params| {
    let player_id: rlua::String = lua_ctx.unpack_multi(params)?;
    let player_id_str = player_id.to_str()?;
    let net = api_ctx.net_ref.borrow();

    if let Some(player) = net.get_player(player_id_str) {
      let table = lua_ctx.create_table()?;
      table.set("texture_path", player.texture_path.as_str())?;
      table.set("animation_path", player.animation_path.as_str())?;

      lua_ctx.pack_multi(table)
    } else {
      Err(create_player_error(player_id_str))
    }
  });

  lua_api.add_dynamic_function("Net", "set_player_avatar", |api_ctx, lua_ctx, params| {
    let (player_id, texture_path, animation_path): (rlua::String, String, String) =
      lua_ctx.unpack_multi(params)?;
    let player_id_str = player_id.to_str()?;

    let mut net = api_ctx.net_ref.borrow_mut();

    net.set_player_avatar(player_id_str, texture_path, animation_path);

    lua_ctx.pack_multi(())
  });

  lua_api.add_dynamic_function("Net", "set_player_emote", |api_ctx, lua_ctx, params| {
    let (player_id, emote_id): (rlua::String, u8) = lua_ctx.unpack_multi(params)?;
    let player_id_str = player_id.to_str()?;

    let mut net = api_ctx.net_ref.borrow_mut();

    net.set_player_emote(player_id_str, emote_id);

    lua_ctx.pack_multi(())
  });

  lua_api.add_dynamic_function(
    "Net",
    "exclusive_player_emote",
    |api_ctx, lua_ctx, params| {
      let (target_id, emoter_id, emote_id): (rlua::String, rlua::String, u8) =
        lua_ctx.unpack_multi(params)?;
      let (target_id_str, emoter_id_str) = (target_id.to_str()?, emoter_id.to_str()?);

      let mut net = api_ctx.net_ref.borrow_mut();

      net.exclusive_player_emote(target_id_str, emoter_id_str, emote_id);

      lua_ctx.pack_multi(())
    },
  );

  lua_api.add_dynamic_function("Net", "animate_player", |api_ctx, lua_ctx, params| {
    let (player_id, name, loop_option): (rlua::String, rlua::String, Option<bool>) =
      lua_ctx.unpack_multi(params)?;
    let (player_id_str, name_str) = (player_id.to_str()?, name.to_str()?);

    let mut net = api_ctx.net_ref.borrow_mut();

    let loop_animation = loop_option.unwrap_or_default();

    net.animate_player(player_id_str, name_str, loop_animation);

    lua_ctx.pack_multi(())
  });

  lua_api.add_dynamic_function(
    "Net",
    "animate_player_properties",
    |api_ctx, lua_ctx, params| {
      use super::actor_property_animaton::parse_animation;

      let (player_id, keyframe_tables): (rlua::String, Vec<rlua::Table>) =
        lua_ctx.unpack_multi(params)?;
      let player_id_str = player_id.to_str()?;

      let mut net = api_ctx.net_ref.borrow_mut();

      let animation = parse_animation(keyframe_tables)?;
      net.animate_player_properties(player_id_str, animation);

      lua_ctx.pack_multi(())
    },
  );

  lua_api.add_dynamic_function("Net", "is_player_in_widget", |api_ctx, lua_ctx, params| {
    let player_id: rlua::String = lua_ctx.unpack_multi(params)?;
    let player_id_str = player_id.to_str()?;

    let net = api_ctx.net_ref.borrow();

    let is_in_widget = net.is_player_in_widget(player_id_str);

    lua_ctx.pack_multi(is_in_widget)
  });

  lua_api.add_dynamic_function("Net", "is_player_busy", |api_ctx, lua_ctx, params| {
    let player_id: rlua::String = lua_ctx.unpack_multi(params)?;
    let player_id_str = player_id.to_str()?;

    let net = api_ctx.net_ref.borrow();

    let is_in_widget = net.is_player_in_widget(player_id_str);

    lua_ctx.pack_multi(is_in_widget)
  });

  lua_api.add_dynamic_function(
    "Net",
    "provide_asset_for_player",
    |api_ctx, lua_ctx, params| {
      let (player_id, asset_path): (rlua::String, rlua::String) = lua_ctx.unpack_multi(params)?;
      let (player_id_str, asset_path_str) = (player_id.to_str()?, asset_path.to_str()?);

      let mut net = api_ctx.net_ref.borrow_mut();

      net.preload_asset_for_player(player_id_str, asset_path_str);

      lua_ctx.pack_multi(())
    },
  );

  lua_api.add_dynamic_function(
    "Net",
    "play_sound_for_player",
    |api_ctx, lua_ctx, params| {
      let (player_id, asset_path): (rlua::String, rlua::String) = lua_ctx.unpack_multi(params)?;
      let (player_id_str, asset_path_str) = (player_id.to_str()?, asset_path.to_str()?);

      let mut net = api_ctx.net_ref.borrow_mut();

      net.play_sound_for_player(player_id_str, asset_path_str);

      lua_ctx.pack_multi(())
    },
  );

  lua_api.add_dynamic_function(
    "Net",
    "exclude_object_for_player",
    |api_ctx, lua_ctx, params| {
      let (player_id, object_id): (rlua::String, u32) = lua_ctx.unpack_multi(params)?;
      let player_id_str = player_id.to_str()?;

      let mut net = api_ctx.net_ref.borrow_mut();

      net.exclude_object_for_player(player_id_str, object_id);

      lua_ctx.pack_multi(())
    },
  );

  lua_api.add_dynamic_function(
    "Net",
    "include_object_for_player",
    |api_ctx, lua_ctx, params| {
      let (player_id, object_id): (rlua::String, u32) = lua_ctx.unpack_multi(params)?;
      let player_id_str = player_id.to_str()?;

      let mut net = api_ctx.net_ref.borrow_mut();

      net.include_object_for_player(player_id_str, object_id);

      lua_ctx.pack_multi(())
    },
  );

  lua_api.add_dynamic_function("Net", "move_player_camera", |api_ctx, lua_ctx, params| {
    let (player_id, x, y, z, duration): (rlua::String, f32, f32, f32, Option<f32>) =
      lua_ctx.unpack_multi(params)?;
    let player_id_str = player_id.to_str()?;

    let mut net = api_ctx.net_ref.borrow_mut();

    net.move_player_camera(player_id_str, x, y, z, duration.unwrap_or_default());

    lua_ctx.pack_multi(())
  });

  lua_api.add_dynamic_function("Net", "slide_player_camera", |api_ctx, lua_ctx, params| {
    let (player_id, x, y, z, duration): (rlua::String, f32, f32, f32, f32) =
      lua_ctx.unpack_multi(params)?;
    let player_id_str = player_id.to_str()?;

    let mut net = api_ctx.net_ref.borrow_mut();

    net.slide_player_camera(player_id_str, x, y, z, duration);

    lua_ctx.pack_multi(())
  });

  lua_api.add_dynamic_function("Net", "unlock_player_camera", |api_ctx, lua_ctx, params| {
    let player_id: rlua::String = lua_ctx.unpack_multi(params)?;
    let player_id_str = player_id.to_str()?;

    let mut net = api_ctx.net_ref.borrow_mut();

    net.unlock_player_camera(player_id_str);

    lua_ctx.pack_multi(())
  });

  lua_api.add_dynamic_function("Net", "lock_player_input", |api_ctx, lua_ctx, params| {
    let player_id: rlua::String = lua_ctx.unpack_multi(params)?;
    let player_id_str = player_id.to_str()?;

    let mut net = api_ctx.net_ref.borrow_mut();

    net.lock_player_input(player_id_str);

    lua_ctx.pack_multi(())
  });

  lua_api.add_dynamic_function("Net", "unlock_player_input", |api_ctx, lua_ctx, params| {
    let player_id: rlua::String = lua_ctx.unpack_multi(params)?;
    let player_id_str = player_id.to_str()?;

    let mut net = api_ctx.net_ref.borrow_mut();

    net.unlock_player_input(player_id_str);

    lua_ctx.pack_multi(())
  });

  lua_api.add_dynamic_function("Net", "teleport_player", |api_ctx, lua_ctx, params| {
    let (player_id, warp, x, y, z, direction_option): (
      rlua::String,
      bool,
      f32,
      f32,
      f32,
      Option<String>,
    ) = lua_ctx.unpack_multi(params)?;
    let player_id_str = player_id.to_str()?;

    let mut net = api_ctx.net_ref.borrow_mut();

    let direction = Direction::from(direction_option.unwrap_or_default().as_str());

    net.teleport_player(player_id_str, warp, x, y, z, direction);

    lua_ctx.pack_multi(())
  });

  lua_api.add_dynamic_function("Net", "message_player", |api_ctx, lua_ctx, params| {
    let (player_id, message, mug_texture_path, mug_animation_path): (
      rlua::String,
      rlua::String,
      Option<String>,
      Option<String>,
    ) = lua_ctx.unpack_multi(params)?;
    let (player_id_str, message_str) = (player_id.to_str()?, message.to_str()?);

    let mut net = api_ctx.net_ref.borrow_mut();

    if let Some(tracker) = api_ctx
      .widget_tracker_ref
      .borrow_mut()
      .get_mut(player_id_str)
    {
      tracker.track_textbox(api_ctx.script_path.clone());

      net.message_player(
        player_id_str,
        message_str,
        &mug_texture_path.unwrap_or_default(),
        &mug_animation_path.unwrap_or_default(),
      );
    }

    lua_ctx.pack_multi(())
  });

  lua_api.add_dynamic_function("Net", "question_player", |api_ctx, lua_ctx, params| {
    let (player_id, message, mug_texture_path, mug_animation_path): (
      rlua::String,
      rlua::String,
      Option<String>,
      Option<String>,
    ) = lua_ctx.unpack_multi(params)?;
    let (player_id_str, message_str) = (player_id.to_str()?, message.to_str()?);

    let mut net = api_ctx.net_ref.borrow_mut();

    if let Some(tracker) = api_ctx
      .widget_tracker_ref
      .borrow_mut()
      .get_mut(player_id_str)
    {
      tracker.track_textbox(api_ctx.script_path.clone());

      net.question_player(
        player_id_str,
        message_str,
        &mug_texture_path.unwrap_or_default(),
        &mug_animation_path.unwrap_or_default(),
      );
    }

    lua_ctx.pack_multi(())
  });

  lua_api.add_dynamic_function("Net", "quiz_player", |api_ctx, lua_ctx, params| {
    let (player_id, option_a, option_b, option_c, mug_texture_path, mug_animation_path): (
      rlua::String,
      Option<String>,
      Option<String>,
      Option<String>,
      Option<String>,
      Option<String>,
    ) = lua_ctx.unpack_multi(params)?;
    let player_id_str = player_id.to_str()?;

    let mut net = api_ctx.net_ref.borrow_mut();

    if let Some(tracker) = api_ctx
      .widget_tracker_ref
      .borrow_mut()
      .get_mut(player_id_str)
    {
      tracker.track_textbox(api_ctx.script_path.clone());

      net.quiz_player(
        player_id_str,
        &option_a.unwrap_or_default(),
        &option_b.unwrap_or_default(),
        &option_c.unwrap_or_default(),
        &mug_texture_path.unwrap_or_default(),
        &mug_animation_path.unwrap_or_default(),
      );
    }

    lua_ctx.pack_multi(())
  });

  lua_api.add_dynamic_function("Net", "open_board", |api_ctx, lua_ctx, params| {
    use crate::net::BbsPost;

    let (player_id, name, color_table, post_tables): (
      rlua::String,
      String,
      rlua::Table,
      Vec<rlua::Table>,
    ) = lua_ctx.unpack_multi(params)?;
    let player_id_str = player_id.to_str()?;

    let mut net = api_ctx.net_ref.borrow_mut();

    if let Some(tracker) = api_ctx
      .widget_tracker_ref
      .borrow_mut()
      .get_mut(player_id_str)
    {
      tracker.track_board(api_ctx.script_path.clone());

      let color = (
        color_table.get("r")?,
        color_table.get("g")?,
        color_table.get("b")?,
      );

      let mut posts = Vec::new();
      posts.reserve(post_tables.len());

      for post_table in post_tables {
        let read: Option<bool> = post_table.get("read")?;
        let title: Option<String> = post_table.get("title")?;
        let author: Option<String> = post_table.get("author")?;

        posts.push(BbsPost {
          id: post_table.get("id")?,
          read: read.unwrap_or_default(),
          title: title.unwrap_or_default(),
          author: author.unwrap_or_default(),
        });
      }

      net.open_board(player_id_str, name, color, posts);
    }

    lua_ctx.pack_multi(())
  });

  lua_api.add_dynamic_function("Net", "prepend_posts", |api_ctx, lua_ctx, params| {
    use crate::net::BbsPost;

    let (player_id, post_tables, reference): (rlua::String, Vec<rlua::Table>, Option<String>) =
      lua_ctx.unpack_multi(params)?;
    let player_id_str = player_id.to_str()?;

    let mut net = api_ctx.net_ref.borrow_mut();

    let mut posts = Vec::new();
    posts.reserve(post_tables.len());

    for post_table in post_tables {
      let read: Option<bool> = post_table.get("read")?;
      let title: Option<String> = post_table.get("title")?;
      let author: Option<String> = post_table.get("author")?;

      posts.push(BbsPost {
        id: post_table.get("id")?,
        read: read.unwrap_or_default(),
        title: title.unwrap_or_default(),
        author: author.unwrap_or_default(),
      });
    }

    net.prepend_posts(player_id_str, reference, posts);

    lua_ctx.pack_multi(())
  });

  lua_api.add_dynamic_function("Net", "append_posts", |api_ctx, lua_ctx, params| {
    use crate::net::BbsPost;

    let (player_id, post_tables, reference): (rlua::String, Vec<rlua::Table>, Option<String>) =
      lua_ctx.unpack_multi(params)?;
    let player_id_str = player_id.to_str()?;

    let mut net = api_ctx.net_ref.borrow_mut();

    let mut posts = Vec::new();
    posts.reserve(post_tables.len());

    for post_table in post_tables {
      let read: Option<bool> = post_table.get("read")?;
      let title: Option<String> = post_table.get("title")?;
      let author: Option<String> = post_table.get("author")?;

      posts.push(BbsPost {
        id: post_table.get("id")?,
        read: read.unwrap_or_default(),
        title: title.unwrap_or_default(),
        author: author.unwrap_or_default(),
      });
    }

    net.append_posts(player_id_str, reference, posts);

    lua_ctx.pack_multi(())
  });

  lua_api.add_dynamic_function("Net", "remove_post", |api_ctx, lua_ctx, params| {
    let (player_id, post_id): (rlua::String, rlua::String) = lua_ctx.unpack_multi(params)?;
    let (player_id_str, post_id_str) = (player_id.to_str()?, post_id.to_str()?);

    let mut net = api_ctx.net_ref.borrow_mut();

    net.remove_post(player_id_str, post_id_str);

    lua_ctx.pack_multi(())
  });

  lua_api.add_dynamic_function("Net", "close_bbs", |api_ctx, lua_ctx, params| {
    let player_id: rlua::String = lua_ctx.unpack_multi(params)?;
    let player_id_str = player_id.to_str()?;

    let mut net = api_ctx.net_ref.borrow_mut();

    net.close_bbs(player_id_str);

    lua_ctx.pack_multi(())
  });

  lua_api.add_dynamic_function("Net", "initiate_pvp", |api_ctx, lua_ctx, params| {
    let (player_1_id, player_2_id, _): (rlua::String, rlua::String, Option<rlua::String>) =
      lua_ctx.unpack_multi(params)?;
    let (player_1_id_str, player_2_id_str) = (player_1_id.to_str()?, player_2_id.to_str()?);

    let mut net = api_ctx.net_ref.borrow_mut();

    net.initiate_pvp(player_1_id_str, player_2_id_str);

    lua_ctx.pack_multi(())
  });

  lua_api.add_dynamic_function("Net", "transfer_player", |api_ctx, lua_ctx, params| {
    let (player_id, area_id, warp_in_option, x_option, y_option, z_option, direction_option): (
      rlua::String,
      rlua::String,
      Option<bool>,
      Option<f32>,
      Option<f32>,
      Option<f32>,
      Option<String>,
    ) = lua_ctx.unpack_multi(params)?;
    let (player_id_str, area_id_str) = (player_id.to_str()?, area_id.to_str()?);

    let mut net = api_ctx.net_ref.borrow_mut();
    let warp_in = warp_in_option.unwrap_or(true);
    let x;
    let y;
    let z;

    if let Some(player) = net.get_player(player_id_str) {
      x = x_option.unwrap_or(player.x);
      y = y_option.unwrap_or(player.y);
      z = z_option.unwrap_or(player.z);
    } else {
      return Err(create_player_error(player_id_str));
    }

    let direction = Direction::from(direction_option.unwrap_or_default().as_str());

    net.transfer_player(player_id_str, area_id_str, warp_in, x, y, z, direction);

    lua_ctx.pack_multi(())
  });

  lua_api.add_dynamic_function("Net", "transfer_server", |api_ctx, lua_ctx, params| {
    let (player_id, address, port, warp_out_option, data_option): (
      rlua::String,
      rlua::String,
      u16,
      Option<bool>,
      Option<String>,
    ) = lua_ctx.unpack_multi(params)?;
    let (player_id_str, address_str) = (player_id.to_str()?, address.to_str()?);

    let mut net = api_ctx.net_ref.borrow_mut();

    let warp = warp_out_option.unwrap_or_default();
    let data = data_option.unwrap_or_default();

    net.transfer_server(player_id_str, address_str, port, &data, warp);

    lua_ctx.pack_multi(())
  });

  lua_api.add_dynamic_function("Net", "kick_player", |api_ctx, lua_ctx, params| {
    let (player_id, reason, warp_out_option): (rlua::String, rlua::String, Option<bool>) =
      lua_ctx.unpack_multi(params)?;
    let (player_id_str, reason_str) = (player_id.to_str()?, reason.to_str()?);

    let mut net = api_ctx.net_ref.borrow_mut();

    let warp_out = warp_out_option.unwrap_or(true);

    net.kick_player(player_id_str, reason_str, warp_out);

    lua_ctx.pack_multi(())
  });
}
