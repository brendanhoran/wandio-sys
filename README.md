# wandio-sys
Rust system bindings for wandio

## Vendor tarball
The tarball under the vendor directory comes from:
https://github.com/LibtraceTeam/wandio/releases

## System dependencies

The [libbgpstream-sys](https://github.com/brendanhoran/libbgpstream-sys) crate needs wandio configured with http support.
This then pulls in `libcurl` as a build dependency, which leads to the below issues.

At this point in time wandio looks for certain libraries on the host system.   
The most critical one is `curl`, and curl then brings in a few more dependences.   
Looking at my system this currently looks like:    
```
$ ldd /usr/bin/curl
	linux-vdso.so.1 (0x00007ffe3b9f9000)
	libcurl.so.4 => /usr/lib64/libcurl.so.4 (0x00007fef25f2d000)
	libz.so.1 => /usr/lib64/libz.so.1 (0x00007fef25f13000)
	libc.so.6 => /usr/lib64/libc.so.6 (0x00007fef25d42000)
	libcares.so.2 => /usr/lib64/libcares.so.2 (0x00007fef25d29000)
	libnghttp2.so.14 => /usr/lib64/libnghttp2.so.14 (0x00007fef25cf8000)
	libssl.so.1.1 => /usr/lib64/libssl.so.1.1 (0x00007fef25c67000)
	libcrypto.so.1.1 => /usr/lib64/libcrypto.so.1.1 (0x00007fef25800000)
	/lib64/ld-linux-x86-64.so.2 (0x00007fef26027000)

```

A [curl-sys](https://github.com/alexcrichton/curl-rust/tree/main/curl-sys) crate exists, however I was unable
to get wandio to use this crate in a static manner, thus limiting its usefulness.  

Thus, given the static version of `curl-sys` seems to break wandio, I have resorted to using system libraries.   
Syntax for the [static-ssl](https://github.com/alexcrichton/curl-rust/tree/main#building) feature.   

Libraries in use by wandio are:   
```
LIBWANDIO_LIBS=' -lpthread -lbz2 -lz -llzo2 -llzma -lzstd -llz4 -lcurl'
```

The line above is from the `config.log` after the source tree has been configured.   
This information is useful if you are consuming this sys crate as you might need 
your crate to link to the host's versions of the above listed libraries.
