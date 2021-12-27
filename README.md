# oszdl
A simple CLI beatmapset downloader for [osu!](https://osu.ppy.sh/)

## Config
The first time you run the script it will ask for your osu! session cookie, directory to save the maps downloaded and will create a yaml config file in `~/.config/oszdl`

You can add filters to the query, adding `m: "0"` to the `filters` field in the config will display only std mapsets and `s: "any"` will display all maps, but it will sort by ranked, qualified and loved maps first.

## Remarks
- Do not try to download a lot of beatmaps at once, you'll get a 429 http code from the server and might get rate limited if you continue, this tool does **not** manage rate limits.
- **Under any hyposthesis** share the session cookie in your config, this is the same as giving your account to the people you shared it with.

## Getting the osu! session cookie
Make sure you're logged in. Press F12 to open dev tools, go to the network tab then access osu's homepage, look for the register that has home as the `File` field. Scroll down to the request headers and copy the Cookie field to the app.

As mentioned above **never** give away this information.
