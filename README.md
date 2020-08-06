# rapkanalyser

 - [x] apk_file_size
 - [x] apk_download_size
 - [x] apk_summary
 - [x] files_list
 - [x] file_cat
 - [x] manifest_print
 - [x] dex_list
 - [x] dex_references
 - [x] dex_packages

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

## Output

### Dex Packages

```
~/Library/Android/sdk/tools/bin/apkanalyzer dex packages tests/resources/apk/app_with_virtual_entry.apk
P d 27	40	3501	<TOTAL>
P d 27	32	3317	com
P d 27	32	3317	com.example
P d 27	32	3317	com.example.android
P d 27	32	3317	com.example.android.multiproject
P d 5	7	704	com.example.android.multiproject.library
P d 4	4	529	com.example.android.multiproject.library.base
C d 2	2	266	com.example.android.multiproject.library.base.BuildConfig
M d 1	1	64	com.example.android.multiproject.library.base.BuildConfig <clinit>()
M d 1	1	54	com.example.android.multiproject.library.base.BuildConfig <init>()
F d 0	0	20	com.example.android.multiproject.library.base.BuildConfig java.lang.String APPLICATION_ID
F d 0	0	12	com.example.android.multiproject.library.base.BuildConfig java.lang.String BUILD_TYPE
F d 0	0	11	com.example.android.multiproject.library.base.BuildConfig boolean DEBUG
F d 0	0	12	com.example.android.multiproject.library.base.BuildConfig java.lang.String FLAVOR
F d 0	0	12	com.example.android.multiproject.library.base.BuildConfig java.lang.String LIBRARY_PACKAGE_NAME
F d 0	0	12	com.example.android.multiproject.library.base.BuildConfig int VERSION_CODE
F d 0	0	12	com.example.android.multiproject.library.base.BuildConfig java.lang.String VERSION_NAME
C d 1	1	149	com.example.android.multiproject.library.base.R$layout
M d 1	1	50	com.example.android.multiproject.library.base.R$layout <init>()
F d 0	0	15	com.example.android.multiproject.library.base.R$layout int lib2layout
F d 0	0	15	com.example.android.multiproject.library.base.R$layout int liblayout
C d 1	1	114	com.example.android.multiproject.library.base.R
M d 1	1	50	com.example.android.multiproject.library.base.R <init>()
C d 1	3	175	com.example.android.multiproject.library.PersonView
M d 1	1	83	com.example.android.multiproject.library.PersonView <init>(android.content.Context,java.lang.String)
M r 0	1	26	com.example.android.multiproject.library.PersonView void setText(java.lang.CharSequence)
M r 0	1	26	com.example.android.multiproject.library.PersonView void setTextSize(float)
P d 5	7	689	com.example.android.multiproject.library2
P d 4	4	514	com.example.android.multiproject.library2.base
C d 2	2	266	com.example.android.multiproject.library2.base.BuildConfig
M d 1	1	64	com.example.android.multiproject.library2.base.BuildConfig <clinit>()
M d 1	1	54	com.example.android.multiproject.library2.base.BuildConfig <init>()
F d 0	0	20	com.example.android.multiproject.library2.base.BuildConfig java.lang.String APPLICATION_ID
F d 0	0	12	com.example.android.multiproject.library2.base.BuildConfig java.lang.String BUILD_TYPE
F d 0	0	11	com.example.android.multiproject.library2.base.BuildConfig boolean DEBUG
F d 0	0	12	com.example.android.multiproject.library2.base.BuildConfig java.lang.String FLAVOR
F d 0	0	12	com.example.android.multiproject.library2.base.BuildConfig java.lang.String LIBRARY_PACKAGE_NAME
F d 0	0	12	com.example.android.multiproject.library2.base.BuildConfig int VERSION_CODE
F d 0	0	12	com.example.android.multiproject.library2.base.BuildConfig java.lang.String VERSION_NAME
C d 1	1	134	com.example.android.multiproject.library2.base.R$layout
M d 1	1	50	com.example.android.multiproject.library2.base.R$layout <init>()
F d 0	0	15	com.example.android.multiproject.library2.base.R$layout int lib2layout
C d 1	1	114	com.example.android.multiproject.library2.base.R
M d 1	1	50	com.example.android.multiproject.library2.base.R <init>()
C d 1	3	175	com.example.android.multiproject.library2.PersonView2
M d 1	1	83	com.example.android.multiproject.library2.PersonView2 <init>(android.content.Context,java.lang.String)
M r 0	1	26	com.example.android.multiproject.library2.PersonView2 void setText(java.lang.CharSequence)
M r 0	1	26	com.example.android.multiproject.library2.PersonView2 void setTextSize(float)
P d 4	4	403	com.example.android.multiproject.person
C d 2	2	236	com.example.android.multiproject.person.People
M d 1	1	54	com.example.android.multiproject.person.People <init>()
M d 1	1	112	com.example.android.multiproject.person.People java.util.Iterator iterator()
C d 2	2	167	com.example.android.multiproject.person.Person
M d 1	1	67	com.example.android.multiproject.person.Person <init>(java.lang.String)
M d 1	1	50	com.example.android.multiproject.person.Person java.lang.String getName()
F d 0	0	10	com.example.android.multiproject.person.Person java.lang.String name
P d 4	4	403	com.example.android.multiproject.person2
C d 2	2	236	com.example.android.multiproject.person2.People
M d 1	1	54	com.example.android.multiproject.person2.People <init>()
M d 1	1	112	com.example.android.multiproject.person2.People java.util.Iterator iterator()
C d 2	2	167	com.example.android.multiproject.person2.Person
M d 1	1	67	com.example.android.multiproject.person2.Person <init>(java.lang.String)
M d 1	1	50	com.example.android.multiproject.person2.Person java.lang.String getName()
F d 0	0	10	com.example.android.multiproject.person2.Person java.lang.String name
C d 2	3	193	com.example.android.multiproject.MainActivity
M d 1	1	54	com.example.android.multiproject.MainActivity <init>()
M d 1	1	73	com.example.android.multiproject.MainActivity void onCreate(android.os.Bundle)
M r 0	1	26	com.example.android.multiproject.MainActivity void setContentView(int)
C d 2	2	230	com.example.android.multiproject.BuildConfig
M d 1	1	64	com.example.android.multiproject.BuildConfig <clinit>()
M d 1	1	54	com.example.android.multiproject.BuildConfig <init>()
F d 0	0	12	com.example.android.multiproject.BuildConfig java.lang.String APPLICATION_ID
F d 0	0	12	com.example.android.multiproject.BuildConfig java.lang.String BUILD_TYPE
F d 0	0	11	com.example.android.multiproject.BuildConfig boolean DEBUG
F d 0	0	12	com.example.android.multiproject.BuildConfig java.lang.String FLAVOR
F d 0	0	12	com.example.android.multiproject.BuildConfig int VERSION_CODE
F d 0	0	12	com.example.android.multiproject.BuildConfig java.lang.String VERSION_NAME
C d 1	1	134	com.example.android.multiproject.R$drawable
M d 1	1	50	com.example.android.multiproject.R$drawable <init>()
F d 0	0	15	com.example.android.multiproject.R$drawable int ic_launcher
C d 1	1	134	com.example.android.multiproject.R$id
M d 1	1	50	com.example.android.multiproject.R$id <init>()
F d 0	0	15	com.example.android.multiproject.R$id int foo
C d 1	1	164	com.example.android.multiproject.R$layout
M d 1	1	50	com.example.android.multiproject.R$layout <init>()
F d 0	0	15	com.example.android.multiproject.R$layout int lib2layout
F d 0	0	15	com.example.android.multiproject.R$layout int liblayout
F d 0	0	15	com.example.android.multiproject.R$layout int main
C d 1	1	149	com.example.android.multiproject.R$string
M d 1	1	50	com.example.android.multiproject.R$string <init>()
F d 0	0	15	com.example.android.multiproject.R$string int app_name
F d 0	0	15	com.example.android.multiproject.R$string int button_send
C d 1	1	114	com.example.android.multiproject.R
M d 1	1	50	com.example.android.multiproject.R <init>()
P r 0	5	112	java
P r 0	3	66	java.util
C r 0	3	66	java.util.ArrayList
M r 0	1	20	java.util.ArrayList <init>()
M r 0	1	26	java.util.ArrayList boolean add(java.lang.Object)
M r 0	1	20	java.util.ArrayList java.util.Iterator iterator()
P r 0	2	46	java.lang
C r 0	1	20	java.lang.Object
M r 0	1	20	java.lang.Object <init>()
C r 0	1	26	java.lang.Boolean
M r 0	1	26	java.lang.Boolean boolean parseBoolean(java.lang.String)
P r 0	3	72	android
P r 0	2	46	android.app
C r 0	2	46	android.app.Activity
M r 0	1	20	android.app.Activity <init>()
M r 0	1	26	android.app.Activity void onCreate(android.os.Bundle)
P r 0	1	26	android.widget
C r 0	1	26	android.widget.TextView
M r 0	1	26	android.widget.TextView <init>(android.content.Context)
```
