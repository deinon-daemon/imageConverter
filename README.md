# imageConverter
prototype CLI tool to replace popular (broken) python-based image converting tools using the power of Rust!


## example usage as a CLI
```shell
cargo run --release -- input.svg output.webp 256
```
#### OR
```shell
cargo run --release -- input.svg output.ico 256
```

## example usage as a CLI in a python venv 
(e.g. Dockerized app where poppler and other image-lib dependencies cause massive memory leaks)
```python
import subprocess

subprocess.run(["cargo", "run", "--release", "--", "input.svg", "output.webp", "256"]) 
```
