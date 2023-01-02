# ResourceDist
Distribute static resource files using HTTP(s)

# Endpoints
GET /hash/{resource}/{filename} returns the blake3 hash of the given file
GET /file/{resource}/{filename} returns the files content
