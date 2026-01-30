# Benchmarks (WIP)

## System 
**CPU:** Ryzen 9 5900X 12 Core 24 Thread

**RAM:** Corsaire Vengeance DDR4 3200MHz

## Results:

### Optimized pixel color compute
>**Note:** During testing my CPU had a small peak of usage then nothing. This made me conclude that the witing in the output file was inefficient. 

| Scene| Resolution |Aspect Ratio | Render Time multi-threaded (rayon)| Render Time single-threaded|
|---------|---------|---------|-------------|-------------|
| **MockWorld: BigAndSmallSpheres**| 16:9 |1920/1080 (FHD) | 29.1496277s | 46.6394222s|
| **MockWorld: BigAndSmallSpheres**| 16:9 |2560/1440 (QHD) | 51.4293508s | 89.269s|
| **MockWorld: BigAndSmallSpheres**| 16:9 |3840/2160 (4K UHD) | 117.7718249s | 201.2773433s|

> Exemple: Image width is set to 1920 pixels and aspect ratio to 16:9 resulting in a 1920/1080 resolution

### Optimized pixel color compute & file writing 
**Note:** I changed the output to be a bufwriter with a fixed capacity of 1MB. Then I optimized the writing algorythm to take advantage of CPU prefetching.   

| Scene| Resolution |Aspect Ratio | Render Time multi-threaded (rayon)| Render Time single-threaded|
|---------|---------|---------|-------------|-------------|
| **MockWorld: BigAndSmallSpheres**| 16:9 |1920/1080 (FHD) | 1.6729761s | TBD|
| **MockWorld: BigAndSmallSpheres**| 16:9 |2560/1440 (QHD) | 3.1117454s | TBD|
| **MockWorld: BigAndSmallSpheres**| 16:9 |3840/2160 (4K UHD) | 7.2304493s | TBD|

> Exemple: Image width is set to 1920 pixels and aspect ratio to 16:9 resulting in a 1920/1080 resolution
