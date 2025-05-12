# Lead 2.0

Because Lead and Lead_Rs didnt impress me, Lead was in Cpp, and Lead_Rs became sort of a mess.

Current progress: 

1. Added Rays, and bounding boxes, relatively clean for now
2. Added Shape trait, and sphere impl
3. Added placeholder trait/structs for future impls as they are referenced in current trait/sturcts including:
   1.  medium, medium_interface
   2.  bsdf, bssrdf, material,
   3.  areaLight
4. Added camera traits, projective camera trait
   1. Added Orthographic camera
   2. Added of Perspective camera
5. Added Filters
6. Added Film, write image support
7. Next up - Reflection models!