# Command Line Interface Specification

There are commands and options

## Commands

- [status](#status)
- [log](#log)
- [init](#init)
- [commit](#commit)

```
tri status
```

### status

Shows the latest commit and its parent, e. g.

```
crop 100x100
54f85854ca6d77d50bcd5e338e78ce15
-> e330efab74317d4b98eb30b03df73fa6
```

### log

Shows the list of all commits before the current one

```
54f85854ca6d77d50bcd5e338e78ce15  crop 100x100
e330efab74317d4b98eb30b03df73fa6  monochrome
54f85854ca6d77d50bcd5e338e78ce15  blur
e330efab74317d4b98eb30b03df73fa6  crop 10x10
```

### init
