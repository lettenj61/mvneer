# mvneer (em-ve-nier)
Command line client for Maven.org Central Repository REST API

# installation
```
cargo install --git 'https://github.com/lettenj61/mvneer'
```

# usage
```
Mvneer 0.1.0
Command line client for Maven Central REST Search API

USAGE:
    mvneer [OPTIONS] <artifact_id> --group <group_id>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -g, --group <group_id>    A group id of the artifact to search
    -n, --rows <rows>         Set max number of records

ARGS:
    <artifact_id>    An artifact to search
```

### search central with artifact name
```
$ mvneer scala-xml
Found 1 result for [artifact: scala-xml] :
    org.scala-lang:scala-xml: latest version [2.11.0-M4] (versions behind: 1)
```

### search with artifact / group

Add `-g <group_id>` right next to artifact name.

```
$ mvneer 'circe-core_2.11' -g 'io.circe'
Found 1 result for [group: io.circe / artifact: circe-core_2.11] :
    io.circe:circe-core_2.11: latest version [0.7.0] (versions behind: 24)
```

### explore with just a group id

Omit artifact name and use only `-g` option.

```
$ mvneer -g 'com.chuusai' -n 12
Found 31 result for [group: com.chuusai] :
    com.chuusai:shapeless_sjs0.6_2.12: latest version [2.3.2] (versions behind: 1)
    com.chuusai:shapeless_2.12: latest version [2.3.2] (versions behind: 1)
    com.chuusai:shapeless_sjs0.6_2.12.0-RC2: latest version [2.3.2] (versions behind: 1)
    com.chuusai:shapeless_2.12.0-RC2: latest version [2.3.2] (versions behind: 1)
    com.chuusai:shapeless_sjs0.6_2.12.0-M5: latest version [2.3.2] (versions behind: 2)
    com.chuusai:shapeless_2.12.0-M5: latest version [2.3.2] (versions behind: 2)
    com.chuusai:shapeless_sjs0.6_2.11: latest version [2.3.2] (versions behind: 14)
    com.chuusai:shapeless_2.11: latest version [2.3.2] (versions behind: 25)
    com.chuusai:shapeless_sjs0.6_2.10: latest version [2.3.2] (versions behind: 14)
    com.chuusai:shapeless_2.10: latest version [2.3.2] (versions behind: 21)
    com.chuusai:shapeless_sjs0.6_2.12.0-M4: latest version [2.3.1] (versions behind: 3)
    com.chuusai:shapeless_2.12.0-M4: latest version [2.3.1] (versions behind: 4)
( ... Omitting another 19 records)
```

`-n` will limit number of records to fetch.

# future plans
- Generalize library for it can be embedded in another application
- Output library version with dependency format of specific tool(e.g. SBT)

# license
The software released under MIT license.
