# Beam Time Estimator

[![dependency status](https://deps.rs/repo/github/alconley/sps_beam_time_estimator/status.svg)](https://deps.rs/repo/alconley/sps_beam_time_estimator)
[![Build Status](https://github.com/alconley/sps_beam_time_estimator/workflows/CI/badge.svg)](https://github.com/alconley/sps_beam_time_estimator/actions?workflow=CI)

Simple gui to estimate the time to run given the cross section, beam current & proton number, target thickness & molar mass, solid angle of the spectrograph, and the number of counts you want in the peak.

### Running Locally
Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`

### Web 

You can test the app at <https://alconley.github.io/sps_beam_time_estimator/>.