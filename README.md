# app template

This repository should be used to create all apps in the fifthtry marketplace 
ecosystem.

## Instructions For People Using Template

- Replace all instances of XXX with the right name (using replace all).
- Rename the three directories.
- Run `ln -s ../../lets-XXX.fifthtry.site lets-XXX.fifthtry.site` from
  `lets-XXX-template.fifthtry.site/.packages`. Use `git add -f` to ensure that 
  the link is checked in the git repo.  
- On FifthTry create the three sites and make them non-editable.
- Create tokens for each of them and add them to Github secrets, follow the 
  names in `.github/yml` files.
- Mark `lets-XXX.fifthtry.site` as a package on FifthTry.
- Remove everything till this line from README.


# `lets-XXX` - A XXX for sites

You can use this fastn app to XXX.

## Developer Setup

Install `fastn`. For Mac/Linux:

```sh
source <(curl -fsSL https://fastn.com/install.sh)
```

For Windows or for other installation methods checkout [fastn.com/install/][1].

[1]: https://fastn.com/install/

Git clone this repository:

```sh
git clone https://github.com/fifthtry-community/lets-XXX.git  
# or if you have ssh setup
git clone git@github.com:fifthtry-community/lets-XXX.git 
```

### Use `auto.sh`

This repo comes with `scripts/auto.sh`, that you can source from your shell:

```shell
source scripts/auto.sh
```

Once done, you will have a few commands available.

### `run-ui`

This is what you want to run when you are building the UI of the `lets-XXX`
app.

Note: call `update-ui` if you modify dependencies in 
`lets-XXX.fifthtry.site/FASTN.ftd`, and during the initial setup.

```sh
update-ui  # only run this when modifying dependencies and during initial setup
run-ui
```

Once you run it, it will start `fastn` server on 8002, so you can visit
`http://127.0.0.1:8002/storybook/` to see various UI states. 

You can find the code of the UI in `lets-XXX.fifthtry.site/ui` folder, and
the storybook configuration in `lets-XXX.fifthtry.site/ui/storybook` folder.

### `run-template`

You want to run this when you want to test the end to end backend functionality
of `lets-XXX` app. This also has a corresponding `update-template` command
which should be used when you modify dependencies or when setting up for the
first time.

Template code is in `lets-XXX-template.fifthtry.site`.

### `run-www`

Use this (and `update-www`) when you want to test the `lets-XXX` apps public
website, which is stored in `lets-XXX.fifthtry-community.com` folder.


## Licence

This repo is MIT Licensed. See [LICENSE](LICENSE) for more details.
