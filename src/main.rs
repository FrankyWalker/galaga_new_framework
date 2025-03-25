use main::desktop_main;

fn main() {

    #[cfg(not(target_arch = "wasm32"))]
    desktop_main();

  }


