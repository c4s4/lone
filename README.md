# Lone Command

Lone command is a way to ensure two instances of the same command won't run
at the same time.

## Installation

### Unix users (Linux, BSDs and MacOSX)

Unix users may download and install latest *lone* release with command:

```bash
sh -c "$(curl https://sweetohm.net/dist/lone/install)"
```

If *curl* is not installed on you system, you might run:

```bash
sh -c "$(wget -O - https://sweetohm.net/dist/lone/install)"
```

**Note:** Some directories are protected, even as *root*, on **MacOSX** (since *El Capitan* release), thus you can't install *lone* in */usr/bin* for instance.

### Binary package

Otherwise, you can download latest binary archive at <https://github.com/c4s4/lone/releases>. Unzip the archive, put the binary of your platform somewhere in your *PATH* and rename it *lone*.

## Usage

To ensure that command *command args...* only runs once at a time, you would type:

```bash
lone 1234 cmd args
```

Where:

- *1234* is a port number that should be the same to given command won't run twice at the same time. For instance, if you run command with port *1234* and *2345*, *lone* won't prevent second instance from running as ports are different. Port must be greater than *1024* if not running as root.
- *cmd args* is the command to run with its arguments.

This command will:

- Open a server socket on given port *1234*. So that if another lone command is already listening this port, this will fail.
- Run given command.
- Release the port when done.

## Shell

To run command in a shell, you would type:

```bash
$ lone --shell 1234 'command args...'
```

In this case, command `cmd args...` will be run in a shell. This enables environment variables evaluation. For instance:

```bash
$ TEST=test lone 1234 echo $TEST
$ TEST=test lone --shell 1234 'echo $TEST'
test
```

On Unix, *lone* will run command in a shell with `sh -c command args...` and `cmd /c command args...` on Windows.

*Enjoy!*
