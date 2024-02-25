# Hibernation Control

:warning:

> As of now, this little tool only works on Ubuntu. There is a lot of hard-coded paths, and it will
> override the Ubuntu's standard swapfile.
>
> Eventually, I might add some more configuration options.

:warning:

## Usage

```sh
cargo build --release
sudo ./target/release/hibernation-control enable
```

## Steam Deck

You will need a manual step to get it working with the Steam Deck. Add `resume` to `HOOKS` at `/etc/mkinitcpio.conf`

Then you can execute:

```sh
sudo ./hibernate-control enable -s /path-to-your-swapfile
```

If don't want the new swapfile to be used for swap, you can run:

_TODO_ set priority
_TODO_ persist swap configuration (fstab or a systemd service)
_TODO_ How do I add it to the menu?

```sh
sudo ./hibernate-control enable -s /path-to-your-swapfile --swapfile-size 16_000_000
```

## Features

- Set up a swapfile of appropriate size (2 time the RAM)
- Configure GRUB to use the swapfile to resume
- Update grub and initramfs
- Configure systemd to hibernate when closing the lid when using battery power

## FAQ

### Hibernate works but suspend-then-hibernate doesn't, why?

systemd has changed the behaviour for suspend-than-hibernate in `252`, but it has been
reverted in `255`. So `HibernateDelaySec` will have different behaviour depending on
your current version.

> See https://github.com/systemd/systemd/issues/25269 and https://github.com/systemd/systemd/issues/25356

From systemd 252 release notes:

> When performing suspend-then-hibernate, the system will estimate the
> discharge rate and use that to set the delay until hibernation and
> hibernate immediately instead of suspending when running from a
> battery and the capacity is below 5%.

From systemd 255 release notes:

> systemd-sleep 'HibernateDelaySec=' setting is changed back to
> pre-v252's behaviour, and a new 'SuspendEstimationSec=' setting is
> added to provide the new initial value for the new automated battery
> estimation functionality. If 'HibernateDelaySec=' is set to any value,
> the automated estimate (and thus the automated hibernation on low
> battery to avoid data loss) functionality will be disabled.

## TODO

- [ ] `disable` command
- [ ] EFI configuration
- [ ] Release binary on github
