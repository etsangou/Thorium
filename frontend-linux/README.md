# Synoptis

Client linux utilisant

# Description

Le code de la client linux de ce projet se divise en 2 parties, l'application (.exe) et le Core (les .dll). Cette architechture permette une meilleurs fludité et une meilleur maintient en contenarisant les différentes parties sensible du client.

# Depandance du projet :

```
sudo apt install cmake
sudo apt install git
sudo apt install build-essential   #(g++, gcc, gdb)
lsudo apt install ibglfw3-dev libglew-dev libgl1-mesa-dev  # Bibliothèque
```

# Compilation :

```
cmake -B build -S .
cmake --build build
./build/Thorium 
```