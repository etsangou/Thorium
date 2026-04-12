cmake_minimum_required(VERSION 3.10)
project(Thorium)

set(CMAKE_CXX_STANDARD 17)

# Find System Graphics Libraries
find_package(glfw3 REQUIRED)
find_package(OpenGL REQUIRED)
find_package(GLEW REQUIRED)

# Collect the ImGui files we downloaded
set(IMGUI_SRCS 
    src/imgui/imgui.cpp 
    src/imgui/imgui_draw.cpp 
    src/imgui/imgui_widgets.cpp 
    src/imgui/imgui_tables.cpp
    src/imgui/backends/imgui_impl_glfw.cpp
    src/imgui/backends/imgui_impl_opengl3.cpp
)

add_executable(Thorium
    main.cpp
    app/app.cpp
    ${IMGUI_SRCS}
)

# Tell the compiler where the headers are
target_include_directories(Thorium PRIVATE src/imgui src/imgui/backends)

# Link the graphics engine
target_link_libraries(Thorium PRIVATE 
    glfw 
    GLEW::GLEW
    OpenGL::GL
)

# Compilation :
# cmake -B build -S .
# cmake --build build
# ./build/Thorium 
