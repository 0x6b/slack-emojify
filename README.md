# slack_emojify

A quick and dirty Slack emoji (`:canned_food:`) to Unicode emoji (🥫) converter.

## Acknowledgement

- [iamcal/emoji-data: Easy to parse data and spritesheets for emoji](https://github.com/iamcal/emoji-data)

## Update Emoji Data

```console
$ cargo x build-emoji-table
$ cargo set-version --bump patch --package slack-emojify
```

## License

MIT. See [LICENSE](LICENSE) for details.

## Privacy

The conversion is solely done locally. The crate never sends user action/data to any server.
