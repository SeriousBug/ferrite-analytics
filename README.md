# Basalt Analytics

This is a service for tracking user events in a way that respects users privacy.

This is currently not ready for general use yet. In the meantime, consider some of these other open source projects:

- [oxitraffic](https://codeberg.org/mo8it/oxitraffic) tracks page visits only, it's simple but effective. Open source, self hosted.
- [Plausible Analytics](https://plausible.io/) is a more comprehensive web analytics tool. It's open source, with both managed service and self hosting options available.

## Running

Make sure you cloned the submodules, otherwise it will not build or run.

## License

All code is freely available under GNU Affero General Public License v3.0.
Please see `LICENSE.txt` for details. The only exception to this is the tracker
script, located under the `tracker` folder. This script is designed to be
embedded into websites and applications, and is licensed under MIT. Please see
the `LICENSE.txt` in that folder.

**What this means for you:** You can deploy Basalytics for your own use, and add
the tracking script to your own website or application. There is nothing special
you need to do, you're free to use it however you want!

However, if you provide Basalytics as a service for others, or you sell or
otherwise redistribute Basalytics, you are bound by the AGPLv3 license. You will
need to release the source code of Basalytics as it is used in your service or
product, along with any modifications you made, and along with any other code
that links with Basalytics code, under GNU Affero General Public License v3.0.
Please see the license text for details.
