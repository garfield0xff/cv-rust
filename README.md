# ORBSLAM 


# Project
This project utilizes ORB-SLAM’s feature-based SLAM approach with an RGB camera to extract edge lines instead of feature points and employs a ToF LiDAR sensor to build the SLAM map. 

# Goal
The goal is to integrate edge lines from the RGB camera with distance data from the ToF LiDAR sensor to create a simplified 3D SLAM solution that accurately reflects structural contours and depth information.

- [x] Harris Corner Detection
- [x] Fast Descriptor
- [ ] Line Segment
 
## Build
 ```bash
 cd /projects/directory
 cargo build
 ```



