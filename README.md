# app template

This repository should be used to create all apps in the fifthtry marketplace 
ecosystem.

## Instructions For People Using Template

- On FifthTry create the three sites and make them non-editable.
- Create tokens for each of them and add them to Github secrets, follow the 
  names in `.github/yml` files.
- Mark `lets-network.fifthtry.site` as a package on FifthTry.
- Remove everything till this line from README.


# `lets-network` - A XXX for sites

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
git clone https://github.com/fifthtry-community/lets-network.git  
# or if you have ssh setup
git clone git@github.com:fifthtry-community/lets-network.git 
```

### Use `auto.sh`

This repo comes with `scripts/auto.sh`, that you can source from your shell:

```shell
source scripts/auto.sh
```

Once done, you will have a few commands available.

### `run-ui`

This is what you want to run when you are building the UI of the `lets-network`
app.

Note: call `update-ui` if you modify dependencies in 
`lets-network.fifthtry.site/FASTN.ftd`, and during the initial setup.

```sh
update-ui  # only run this when modifying dependencies and during initial setup
run-ui
```

Once you run it, it will start `fastn` server on 8002, so you can visit
`http://127.0.0.1:8002/storybook/` to see various UI states. 

You can find the code of the UI in `lets-network.fifthtry.site/ui` folder, and
the storybook configuration in `lets-network.fifthtry.site/ui/storybook` folder.

### `run-template`

You want to run this when you want to test the end to end backend functionality
of `lets-network` app. This also has a corresponding `update-template` command
which should be used when you modify dependencies or when setting up for the
first time.

Template code is in `lets-network-template.fifthtry.site`.

### `run-www`

Use this (and `update-www`) when you want to test the `lets-network` apps public
website, which is stored in `lets-network.fifthtry-community.com` folder.


## Licence

This repo is MIT Licensed. See [LICENSE](LICENSE) for more details.
