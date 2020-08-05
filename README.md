# rapkanalyser

 - [x] apk file-size
 - [x] apk download-size

## Diff

```
~/Library/Android/sdk/tools/bin/apkanalyzer apk download-size /Users/fdhuang/works/android/rapkanalyser/tests/resources/apk/app_with_virtual_entry.apk
39633
```

`39633` vs `39591`

## Document

```
~/Library/Android/sdk/tools/bin/apkanalyzer

Subject must be one of: apk, files, manifest, dex, resources

apk summary              Prints the application Id, version code and version name.
apk file-size            Prints the file size of the APK.
apk download-size        Prints an estimate of the download size of the APK.
apk features             Prints features used by the APK.
apk compare              Compares the sizes of two APKs.
files list               Lists all files in the zip.
files cat                Prints the given file contents to stdout
manifest print           Prints the manifest in XML format
manifest application-id  Prints the application id.
manifest version-name    Prints the version name.
manifest version-code    Prints the version code.
manifest min-sdk         Prints the minimum sdk.
manifest target-sdk      Prints the target sdk
manifest permissions     Prints a list of used permissions
manifest debuggable      Prints if the app is debuggable
dex list                 Prints a list of dex files in the APK
dex references           Prints number of references in dex files
dex packages             Prints the class tree from DEX.
                         P,C,M,F: indicates packages, classes methods, fields
                         x,k,r,d: indicates removed, kept, referenced and defined
                           nodes
dex code                 Prints the bytecode of a class or method in smali format
resources packages       Prints a list of packages in resources table
resources configs        Prints a list of configurations for a type
resources value          Prints the given resource's value
resources names          Prints a list of resource names for a type
resources xml            Prints the human readable form of a binary XML

Usage:
apkanalyzer [global options] <subject> <verb> [options] <apk> [<apk2>]

Option            Description
------            -----------
--human-readable  Print sizes in human readable format
```

## Dump

```
aapt2 dump badging tests/resources/apk/app_with_virtual_entries.apk
```
