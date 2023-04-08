# Playlist Laser aka Playzer

It is a law of nature, playlists always become filled with duplicate and deleted files. With _Playzer_ that no longer has to be the case. Built with Rust, this blazingly-fast<sup>TM</sup> CLI tool lets you get ahold of your unwieldly playlists. All you have to do is download the binary in the releases tab and run it!

## Instructions
### Using the Binary

Run the executable and follow the instructions. You can also provide arguments to the executable.  
1. First argument is the path to the playlist you want to clean, absolute or relative. If there are spaces, it has to be surrounded with double quotes.
1. Second is the output format: m3u, pls or asx.
1. Third is options. Order doesn't matter.
   * s - shuffle
   * d - keep duplicates

Example   
```.\playzer "Y:\music.pls" m3u ds```

### Using Cargo
Alternatively, if you have rust installed, you can clone this repo and supply the above-mentioned arguments to ```cargo run --```.  
Example   
```cargo run -- Y:\music.pls m3u ds"```