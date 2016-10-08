chk-chglog
==========

Do you build RPMs ? 

Do you have the good sense to include a Changelog ?

Are you human ?

Well, you've probably screwed up the Changelog at some point then. Shame it makes it all the way to the build system before something is nice enough to let you know there's a problem...

This is a teeeeeny tiny little [Rust](https://www.rustlang.org) experiment to perform a lickity-split check of the dates and versions listed in your Changelog file to ensure everything is in the correct order and format.

```
  $ chk-chglog Changelog
```

I recommend placing it into your ```pre-commit hook``` to stop silly mistakes from ever making it in. It's simple and swift, you won't know it's there.

Currently it only works on Changelogs that are formatted as per the Fedora RPM specs, if anyone is interested enough to raise an issue, I should be able to make it work with other structures.

Maybe someone else might find it useful, who knows!

Rust-y bits...
--------------

I don't really like the ```panic!``` and other error handling bits I have in there. But thus far I haven't found a way I'm comfortable with. The experiments continue.. and...

Yes dibblego, I'm sitting with my Strings to the door. I heard you, Dr Yueh, and Gurney coming down the hall.
