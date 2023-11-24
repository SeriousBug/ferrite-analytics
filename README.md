# Ferrite Analytics

This is a service for tracking user events in a way that respects users' privacy.

### What is Ferrite Analytics?

Ferrite Analytics tracks page visits and other events on websites like items on
pages being clicked, hovered over, or being scrolled into view. It also tracks
user sessions (limited to a single day). All of this is done without storing any
cookies or other data on users devices, and without collecting any personally
identifiable information like IP addresses.

This tracking can be done either with a tracking pixel, which tracks views only,
or a small tracking script that comes under 2KB uncompressed, which can be
configured to track many things.

### How does Ferrite Analytics respect privacy?

To respect users privacy, Ferrite Analytics doesn't save users IP addresses, or
even full user agents. It only saves a summary of the user agent, like "Windows,
Firefox", and a session hash.

The session hash is computed from the users IP address, user agent, the current
day, along with a throwaway "day code" as salt. This was inspired by Plausible
Analytics, who discovered that the IP address and user agent are sufficient to
track user sessions effectively. Adding the current day (like `2023-08-06`) to
this limits tracked sessions to a day, ensuring users are not tracked over a
long period of time. Finally the "day code" is a randomly generated code that is
kept in memory and thrown away every day. This ensures that it is extremely
difficult, if not impossible, to reverse the hashing process by brute force as
an attacker would not be able to guess the day code.

### Can I use Ferrite Analytics?

Not really. It's not ready for general use yet. While the tracking is
functional, the dashboard is still in progress meaning you can't really view the
collected events or analyze them in any way. And configuring the tracker script
requires writing json by hand.

In the meantime, please consider some of these other open source projects:

- [oxitraffic](https://codeberg.org/mo8it/oxitraffic) tracks page visits and
  time spent only, it's simple but effective. Open source, self hosted.
- [Plausible Analytics](https://plausible.io/) is a more comprehensive web
  analytics tool. It's open source, with both managed service and self hosting
  options available.

### Do I need a cookie banner / GDPR consent banner with Ferrite Analytics?

Yes, you probably still do. If you read [the guidelines by the European Data Protection Board](https://edpb.europa.eu/our-work-tools/documents/public-consultations/2023/guidelines-22023-technical-scope-art-53-eprivacy_en), they explain that (abbreviated by me):

> ... [tracking] pixel or tracked URL [...] constitutes storage [...], at the very least through the caching mechanism of the client-side software. As such,
> Article 5(3) ePD is applicable ...

In other words: even if you don't store any cookies or unique identifiers, and
you don't access any personal or even identifiable information, you are still in
violation of the ePrivacy Directive if you don't ask for consent first. Because
the tracking script or pixel might be cached and that's technically "information
stored on the terminal equipment" which can't be done without asking for consent
first.

This is a very extreme reading of the ePrivacy directive in my opinion, and one
that relies on a technicality (i.e. if I configure `Cache-Control` headers so
the tracking pixel is not cached, is it okay then?). I also think this is not
what the spirit of the law had intended. Yet, I'm not a lawyer and I'd have to
trust European Data Protection Board's view over my own.

## Usage

If you do want to use Ferrite Analytics, the preferred way to deploy Ferrite
Analytics is using Docker. If you would like to avoid docker, binaries compiled
for different operating systems and architectures are available under the
[Releases tab](https://github.com/SeriousBug/ferrite-analytics/releases). Docker
container images are available both on ghcr.io
(`ghcr.io/seriousbug/ferrite-analytics`)and DockerHub
(`seriousbug/ferrite-analytics`).

To run Ferrite Analytics, you will need to set the environment variable
`DATABASE_URL` to a database. Available options are Sqlite and PostgresQL. For
Sqlite, use `DATABASE_URL=sqlite:/path/to/database.sqlite?mode=rwc`. PostgresQL
is untested right now, so figure it out yourself! Ferrite Analytics will
automatically perform any required migrations when you launch it.

If you are running Ferrite Analytics behind a reverse proxy, you should also set
the `--forward-ip-header` parameter. This parameter should be set to the name of
a header that holds the original IP address. For example, if you have an nginx
config such as:

```
location /ferrite/ {
  proxy_pass http://localhost:3000/;
  proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
}
```

Then you should set the parameter to `X-Forwarded-For` like `--forward-ip-header X-Forwarded-For`.

Here is a `docker-compose.yml` file that puts all of these together:

```yml
version: "3"
services:
  ferrite-analytics:
    # Get container image from ghcr (Github)
    image: ghcr.io/seriousbug/ferrite-analytics:latest
    # Get the IP address from the `X-Forwarded-For` header. Only set this if Ferrite Analytics is behind a reverse proxy!
    command: --forward-ip-header 'X-Forwarded-For'
    restart: unless-stopped
    # Put all the data in a named volume
    volumes:
      - ferrite-analytics-data:/data
    environment:
      - DATABASE_URL=sqlite:/data/ferrite.sqlite?mode=rwc
    # Expose port 3000, where Ferrite Analytics' API is.
    ports:
      - 3000:3000
volumes:
  ferrite-analytics-data:
```

## Developing

Make sure you cloned the submodules, otherwise it will not build or run.

Make sure you have `cargo` and `pnpm` installed.

You will first need to run `pnpm install && pnpm run` inside `tracker` to build the tracker script.
Then, run `cargo run` inside `service` to build and run the service.

To make changes in any database entities, you will need to have `sea-orm-cli`
installed. Add a new migration under `service/migration/src`, then run
`sea-orm-cli migrate up` while inside `service` to migrate the database up. Then
run `sea-orm-cli generate entity -o src/entity` to update the generated entity
code. There is also a VSCode task to regenerate entities.

## License

All code is freely available under GNU Affero General Public License v3.0.
Please see `LICENSE.txt` for details. The only exception to this is the tracker
script, located under the `tracker` folder. This script is designed to be
embedded into websites and applications, and is licensed under MIT. Please see
the `LICENSE.txt` in that folder.

**What this means for you:** You can deploy Ferrite Analytics for your own use, and add
the tracking script to your own website or application. There is nothing special
you need to do, you're free to use it however you want!

However, if you provide Ferrite Analytics as a service for others, or you sell or
otherwise redistribute Ferrite Analytics, you are bound by the AGPLv3 license. You will
need to release the source code of Ferrite Analytics as it is used in your service or
product, along with any modifications you made, and along with any other code
that links with Ferrite Analytics code, under GNU Affero General Public License v3.0.
Please see the license text for details.
