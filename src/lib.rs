use zed_extension_api as zed;

struct MyExtension;

impl zed::Extension for MyExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(MyExtension);
