# servicentral

TBD

<br/>

## Prerequisites & Setup

The followings are the required tools and steps to have the proper setup for the app to run.

### Front-end related

Note: These are needed during development. In other words, if you don't change any code in the components (within `rsx` blocks), then there is no need to run the Tailwind CSS compiler in parallel with the back-end.

1. Install [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm).
2. Install the Tailwind CSS [CLI](https://tailwindcss.com/docs/installation).
3. Start the Tailwind CSS compiler using `./run_css.sh` script.

### Back-end related

TBD

<br/>

### Run

Launch the app using `./run-dev.sh`.

In case of compilation error that remain hidden behind Dioxus CLI, run `cargo check --features server,web` to reveal them.
