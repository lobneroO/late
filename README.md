Late is a simple GUI to set the buffer size (and implicitly latency) and sample rate for pipewire.
While there are great tools like qpwgraph to set up connections, you currently have to go to the
terminal to change the pipewire settings.

Late is meant to make changing this setting straight forward.

You can choose a buffer size from predefined buffer sizes. 
Likewise, you can choose a sample rate from predefined sample rates.

At the moment, no checking is done if your hardware actually supports any of these.

*BEWARE*
If you change these values, while a program is running that uses any of these settings, the running program may crash.
E.g. running ML Sound Lab Amped Roots via wine will crash when changing either buffer size or sample rate.

