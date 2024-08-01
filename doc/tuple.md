
# Tuple expression

## Tuple as module parameters

module box((x,y,z) = 0mm) {

}

module box(x = 0mm, y = 0mm, z = 0mm) {
}

module box(x,y,z = 0mm) {
}

## Field declaration for a module

(width, height) := (1,2)mm;
width := 1.2mm;
height := 2mm;
(width, height) := (0mm,0mm);

width := (0.0, 0.0)mm;
height := (0.0, 0.0)mm;

width, height := 0mm;
