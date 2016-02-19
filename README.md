# RAlly

[ActionAlly](http://actionally.com/) is a very cool mac app that helps improve
your productivity, but it does not provide all the features I wanted out of the
box. So, this is a CLI that helps with controlling ActionAlly more easily and
from the terminal, as a bonus.

## Getting Started

You can install rally in one of two ways:

**Homebrew**

```
brew tap subvisual/formulae
brew install rally
```

**Cargo**

```
cargo install rally
```

Then run `rally help` to see all the commands available.

## Database file

By default `RAlly` tries to figure where your `ActionAlly` database is located,
but if it can't figure it out, or you want to configure it yourself, it can be
overridden by an environment variables:

```
DATABASE_URL=some/path rally
```

Another alternative, is to have a `.env` file with that variable set. The
downside with this approach is that you have to run the command from the
directory where that file is located, every time.
