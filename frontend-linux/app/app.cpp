#include "app.hpp"

app::app(GLFWwindow* window) : _window(window) {
    //std::cout << "obj init" << std::endl;
    this->setup_imgui();
}

app::~app() = default;

void app::start() {
    this->switch_bg_color(ImVec4(0.17f, 0.18f, 0.20f, 1.00f));

    // ImGuiIO& io = ImGui::GetIO();
    // ImFont* font = io.Fonts->AddFontFromFileTTF("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", 18.0f);
    // if (font == nullptr) std::cout << "Could not load font, using default.\n" << std::endl;

    // Boucle principale
    while (!glfwWindowShouldClose(_window)) {
        this->new_frame();


        // Liste de serveur
        ImGui::SetNextWindowPos(ImVec2(0, 0));
        ImGui::SetNextWindowSize(ImVec2(70, 720));
        ImGui::Begin("Sidebar", NULL, ImGuiWindowFlags_NoDecoration);

        // Interieur de la liste
        ImGui::Button("Srv 1", ImVec2(50, 50)); // Bouton icon
        ImGui::Button("Srv 2", ImVec2(50, 50)); // Bouton icon
        ImGui::Button("Srv 3", ImVec2(50, 50)); // Bouton icon

        ImGui::End();


        // Chat
        ImGui::SetNextWindowPos(ImVec2(70, 0));
        ImGui::SetNextWindowSize(ImVec2(1210, 720));

        // Interieur du chat
        ImGui::Begin("Chat", NULL, ImGuiWindowFlags_NoDecoration);
        ImGui::Text("Welcome to the C++ Lounge");
        if (ImGui::Button("Join Voice Channel")) ImGui::Text("Hello");// Bouton avec logique

        ImGui::End();


        this->update_window();
    }
}

void app::new_frame() {
    glfwPollEvents();
    ImGui_ImplOpenGL3_NewFrame();
    ImGui_ImplGlfw_NewFrame();
    ImGui::NewFrame();
}

void app::update_window() {
    ImGui::Render();
    glClear(GL_COLOR_BUFFER_BIT);
    ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());
    glfwSwapBuffers(_window);
}

// Permet de change la couleur de font de la fenêtre
void app::switch_bg_color(ImVec4 color) {
    ImGui::StyleColorsDark();
    auto& style = ImGui::GetStyle();
    style.Colors[ImGuiCol_WindowBg] = color; // Discord Gray
}

// Mise en place des paramêtres de la bibiothèque ImGui
void app::setup_imgui() {
    IMGUI_CHECKVERSION();
    ImGui::CreateContext();
    ImGui_ImplGlfw_InitForOpenGL(_window, true);
    ImGui_ImplOpenGL3_Init("#version 130");
}