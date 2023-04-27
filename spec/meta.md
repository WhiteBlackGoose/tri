# Meta

Refers to file commits

Simple CSV:

```
commit,parent,command,node_status                         
54f85854ca6d77d50bcd5e338e78ce15,,,
54f85854ca6d77d50bcd5e338e78ce15,e330efab74317d4b98eb30b03df73fa6,crop 100x100,
54f85854ca6d77d50bcd5e338e78ce15,e330efab74317d4b98eb30b03df73fa6,monochrome,HEAD
```

For every line one of the following is true:
- It has different commit and parent hashes, non-empty command, HEAD as status
- It only has commit but empty parent, no command and no node_status
- It only has commit but empty parent, no command but node_status is HEAD

# Unresolved features

- Commit message?
- Time?
- Author???
