#pragma once

#include <iostream>
#include <string>

#include <GL/glew.h>
#include <GLFW/glfw3.h>
#include "../src/imgui/imgui.h"
#include "imgui_impl_glfw.h"
#include "imgui_impl_opengl3.h"

class app
{
private:
    GLFWwindow* _window = nullptr;
    void switch_bg_color(ImVec4 color);
    void setup_imgui();
    void new_frame();
    void update_window();

public:
    app(GLFWwindow* window);
    ~app();
    void start();
};