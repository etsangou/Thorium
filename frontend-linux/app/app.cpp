// app.cpp
#include "app.hpp"

void window_refresh_callback(GLFWwindow* window) {
    auto* instance = static_cast<app*>(glfwGetWindowUserPointer(window));
    if (instance) {
        instance->run_frame();
    }
}

app::app(application_specification app_spec) : _app_spec(app_spec) {
    //std::cout << "obj init" << std::endl;

    // Création de la fenêtre
    if (!glfwInit()) throw std::runtime_error("Échec de l'initialisation de GLFW");
    _window = glfwCreateWindow(_app_spec.windows_spec.width, _app_spec.windows_spec.height, _app_spec.name.c_str(), NULL, NULL);
    glfwMakeContextCurrent(_window);

    glfwSetWindowUserPointer(_window, this);
    
    // 3. On définit le callback de rafraîchissement
    glfwSetWindowRefreshCallback(_window, window_refresh_callback);

    this->setup_imgui();
}

app::~app() = default;

void app::start() {
    this->switch_bg_color(ImVec4(0.17f, 0.18f, 0.20f, 1.00f));

    ImGuiIO& io = ImGui::GetIO();
    ImFont* font = io.Fonts->AddFontFromFileTTF("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", 18.0f);
    if (font == nullptr) std::cout << "Could not load font, using default.\n" << std::endl;

    // Boucle principale
    while (!glfwWindowShouldClose(_window)) {
        this->run_frame();
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

void app::run_frame() {
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
    if (ImGui::Button("Join Voice Channel")) { // Bouton avec logique
        web_r.get((std::string)"http://127.0.0.1:5000");
        std::cout << web_r.get_web_answer() << std::endl;
    }

    message_arrage msg_test;
    msg_test.username = "Enzo";
    msg_test.msg = "Salut ! J'ai enfin regle les erreurs de compilation.";
    msg_test.date = "2024-05-20";
    msg_test.hour = "14:35";

        ImGui::BeginChild("ScrollingRegion", ImVec2(0, 0), false, ImGuiWindowFlags_HorizontalScrollbar);
            for(int i = 0; i < 80; i++) {
                message_struct(msg_test);
            }
        ImGui::EndChild();

    ImGui::End();

    this->update_window();
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

void app::message_struct(message_arrage message) {
    ImGui::BeginGroup();
    
    ImGui::TextColored(ImVec4(1.0f, 1.0f, 1.0f, 1.0f), "%s", message.username.c_str());
    ImGui::SameLine();
    ImGui::TextDisabled("aujourd'hui à %s", message.hour.c_str());
    ImGui::TextWrapped("%s", message.msg.c_str());
    
    ImGui::Spacing();
    ImGui::EndGroup();
}