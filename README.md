`cargo-devcade` is a helper for building and deploying devcade games with Rust!

# Installing
Installation is simple:

```
cargo install cargo-devcade
```

_Psst!_ Make sure you also have [`cross`](https://github.com/cross-rs/cross/wiki/Getting-Started) installed and working too!

# Getting started with `cargo-devcade` and `bevy`!

Though it's not required, I recommend you use `bevy` and `devcaders` to put stuff on the screen.

## Project setup
Once you have `cargo-devcade` installed, we can make a new project:

```bash
cargo new ferris-spinner
cd ferris-spinner
```

Now, let's add some dependencies we'll need:

```bash
cargo add bevy devcaders
```

> *Mary Note:*
>
> * [Bevy](https://bevyengine.org) is a game engine for Rust.
> * [devcaders](https://docs.rs/devcaders/latest/devcaders) is a library for adding Devcade functionality to your Bevy game!

Oh, and don't forget the icons! `icon.png` and `banner.png` should exist in the `store_icons` folder of your crate root:

```bash
mkdir store_icons
curl https://placehold.it/800x450.png -Lo store_icons/banner.png
curl https://placehold.it/512x512.png -Lo store_icons/icon.png
```

## Building

Before we do anything else, let's do a test build to make our project is set up properly:

```bash
cargo devcade package
```

This should run a build, and package your app for production.

If it didn't work, maybe you hit a bug... Or more likely you set it up wrong!

## Now, write some freaking code

Bevy is pretty neat, here's a tiny game we can try with:

```rs
use bevy::{prelude::*, window::WindowMode};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        mode: WindowMode::Fullscreen,
        ..default()
      }),
      ..default()
    }))
    .add_startup_system(setup_system)
    .add_system(hello_world_system)
    .run();
}

fn setup_system() {
  println!("Welcome to ferris spin!");
}

fn hello_world_system() {
  println!("hello world");
}
```

Cool, let's give it a run:

```bash
cargo run
```

It does... nothing! ...Except print 'hello world' a bunch of times and make a window.

That's because we're not drawing anything yet!

## Grey is boring

Let's grab some pictures from the interwebz to use in our game:

```bash
mkdir assets
curl https://rustacean.net/assets/rustacean-flat-happy.png -o assets/ferris.png
```

Cool! Now let's make a camera and a ferris at startup:

```rs
#[derive(Component)]
struct Ferris {}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
  println!("Welcome to ferris spin!");
  // Add a camera so we can see stuff:
  commands.spawn(Camera2dBundle::default());
  // Draw a ferris in the middle of the screen
  commands.spawn((
    Ferris {},
    SpriteBundle {
      texture: asset_server.load("ferris.png"),
      transform: Transform::from_xyz(100., 0., 0.),
      ..default()
    },
  ));
}
```

And now:
```
cargo run
```

You should see a crab on your screen. If you don't and got some kind of error... Open an issue?

An interesting bevy pattern showed up here, did you spot it?

### Markers

In bevy, sometimes we link a dummy component (in this case `Ferris`) so it can be looked-up later. We'll see some of that syntax in the next step

## What if the crab actually... Did something?

Hey, this is cool, but what if we could move the crab around?

```rs
fn hello_world_system(mut sprite_position: Query<(&mut Ferris, &mut Transform)>) {
  for (_, mut transform) in &mut sprite_position {
    transform.translation.x -= 1.0;
  }
  println!("hello world");
}
```

But uh... What if we could actually control it?

Hmm... But how? Maybe that `devcaders` library we added earlier could help us...

```rs
use devcaders::{DevcadeControls, Player, Button};

fn hello_world_system(
  time: Res<Time>,
  mut sprite_position: Query<(&mut Ferris, &mut Transform)>,
  devcade_controls: DevcadeControls,
) {
  for (_, mut transform) in &mut sprite_position {
    if devcade_controls.pressed(Player::P1, Button::StickLeft) {
      transform.translation.x -= 5.0 * time.delta_seconds();
    }
    if devcade_controls.pressed(Player::P1, Button::StickRight) {
      transform.translation.x += 5.0 * time.delta_seconds();
    }
  }
  println!("hello world");
}
```

Cool! Now our crab moves when we move the joystick.

For a full list of controls and their meanings, check [devcaders' `Button` enum](https://docs.rs/devcaders/latest/devcaders/enum.Button.html)

If you don't have a controller attached, you can use the `V` and `N` keys instead!

For more information, or to learn more, visit [bevyengine.org](https://bevyengine.org/)!

## One last thing

So... We should make it possible to exit.

Add these lines to our system to exit on menu button press (by either player!):
```rs
if devcade_controls.pressed(Player::P1, Button::Menu)
  || devcade_controls.pressed(Player::P2, Button::Menu)
{
  std::process::exit(0);
}
```

## Publishing

Publishing to devcade is easy:

1. Run `cargo devcade package`
2. Upload the zip to [the devcade upload portal](https://devcade.csh.rit.edu/upload), making sure to set the title to the name of your crate
3. Profit?
