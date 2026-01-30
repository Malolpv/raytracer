# Benchmarks (WIP)

## System 
**CPU:** Ryzen 9 5900X 12 Core 24 Thread

**RAM:** Corsaire Vengeance DDR4 3200MHz

## Results:
> **Note:** During testing my CPU was used at like 5% so something might be wrong with those results


| Scene| Resolution |Aspect Ratio | Render Time multi-threaded (rayon)| Render Time single-threaded|
|---------|---------|---------|-------------|-------------|
| **MockWorld: BigAndSmallSpheres**| 16:9 |1920/1080 (FHD) | 29.1496277s | 46.6394222s|
| **MockWorld: BigAndSmallSpheres**| 16:9 |2560/1440 (QHD) | 51.4293508s | 89.269s|
| **MockWorld: BigAndSmallSpheres**| 16:9 |3840/2160 (4K UHD) | 117.7718249s | 201.2773433s|

> Exemple: Image width is set to 1920 pixels and aspect ratio to 16:9 resulting in a 1920/1080 resolution
