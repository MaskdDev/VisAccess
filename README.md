# VisAccess - A Simple Visual Accessibility Tool for Discord!
VisAccess is a utility bot that allows you to warn members when their images don't have descriptions, whether this be in the alt text or in the contents of the message.

## Deploying VisAccess

### Cargo
For the _simplest_ setup, VisAccess should be all set to run using `cargo run`, using a given Rust installation on your machine.

The following environment variables must be set:
- `TOKEN`: The discord token for the bot user you'd like to run VisAccess on.

The following environment variables can be set:
- `ALL_MESSAGE`: The message you'd like to send when all images in a message don't have a description, and the message has more than one image.
- `SOME_MESSAGE`: The message you'd like to send when some images in a message don't have a description. %n will be replaced with the number of images without a description in the message.
- `SINGULAR_MESSAGE`: The message you'd like to send when a single image in a message doesn't have a description.

### Railway
You can also use the provided railway template below to deploy VisAccess with ease. All configuration options are documented within the template.

[![Deploy on Railway](https://railway.com/button.svg)](https://railway.com/deploy/xDdTf6?referralCode=maskddev)
