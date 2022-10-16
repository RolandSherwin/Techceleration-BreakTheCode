# Techceleration-BreakTheCode
Each solution is a binary, use `cargo run --release --bin <binary_name>` to run a single soltion with default args

#### 1. Categorize Places of Interest (`poi`)
1. Export your Google Maps API key as `export GOOGLE_DEV_API_KEY=<your_key_here>`
2. `cargo run --release --bin poi -- --lat 13.08 --lng 80.27 --radius 20000 --search petrol --category 5`
```
    --lat <LATITUDE>
        Latitude of the location [default: 13.0827]
    --lng <LONGITUDE>
        Longitude of the location [default: 80.2707]
-r, --radius <RADIUS>
        Radius from the location to search for, max 50000m [default: 50000]
-s, --search <SEARCH>
        Item to search for [default: restaurant]
-c, --category <DISTANCE_CATEGORY>
        Distance in KM to categorize the results [default: 2]
```

#### 3. GitHub Contributors Statistics (`contributors`)
`cargo run --release --bin contributors -- --pages 2`
```
-p, --pages <MAX_PAGES>  Number of pages to go through [default: 1]
```
- Uses `tokio::spawn` for each page to fetch them concurrently

#### 4. Filtered News List (`news`)
1. Export your CurrentsAPI.service API key as `export CURRENTS_API_KEY=<your_key_here>`
2. `cargo run --release --bin news`
3. Selecting an option using `<Space>` will include news from that filter. Use `<Enter>` to goto the next category.
