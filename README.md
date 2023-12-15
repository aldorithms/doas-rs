# doas-rs
A Rust port of OpenBSD's doas command.

## Changes from the original
1. **Single Binary**: ``doas-rs`` will be compiled statically, and internally include ``vidoas`` & ``doasedit`` in a single binary. Consequently, in order to use it normally, you will need to symlink or alias ``doas-rs doas`` to ``doas``
2. **Zero Persistence**: Persistence will not be a feature in ``doas-rs`` as a security measure. Instead, you are expected to type in your password everytime, in order to mitigate the risk of an evil maid attack during the time period your password is active. When support for ``doas.conf`` is added, the parser will simply ignore the ``persist`` keyword.
