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

    // --- Zone logo Thorium ---
    ImGui::SetNextWindowPos(ImVec2(0, 0));
    ImGui::SetNextWindowSize(ImVec2(LOGO_ZONE_X_SIZE, LOGO_ZONE_Y_SIZE));
    ImGui::Begin("Thorium", NULL, ImGuiWindowFlags_NoDecoration);

    ImGui::Text("Thorium");
    ImGui::SameLine();
    if (ImGui::Button("U+", ImVec2(20, 20))) { // Ouvre une fenêtre afin d'ajouter un utilisateur
        std::cout << "Ajouter un utilisateur" << std::endl;
    }

    ImGui::End();


    // --- Zone Liste d'utilisateur --
    ImGui::SetNextWindowPos(ImVec2(0, LOGO_ZONE_Y_SIZE));
    ImGui::SetNextWindowSize(ImVec2(CONTACT_ZONE_X_SIZE, CONTACT_ZONE_Y_SIZE));
    ImGui::Begin("Contact list", NULL, ImGuiWindowFlags_NoDecoration);

    std::vector<contact_arrage> contacts = {
        {"alice", false, 0}, 
        {"bob", false, 1}, 
        {"enzo", false, 2}, 
        {"pierre", true, 3}, 
        {"maia", false, 4}, 
        {"la_magnifique_chloé", false, 5}
    };

    ImGui::Text("Contact — %zu",contacts.size());
    ImGui::Spacing();
    for (auto contact : contacts) {
        contact_struct(contact);
    }


    ImGui::End();

    // --- Profil ---
    ImGui::SetNextWindowPos(ImVec2(0, LOGO_ZONE_Y_SIZE + CONTACT_ZONE_Y_SIZE));
    ImGui::SetNextWindowSize(ImVec2(PROFIL_ZONE_X_SIZE, PROFIL_ZONE_Y_SIZE));
    ImGui::Begin("Profil", NULL, ImGuiWindowFlags_NoDecoration);

    if (ImGui::Button("PP", ImVec2(40, 40))) { // Bouton avec logique
        std::cout << "Afficher le QRcode de l'utilisateur" << std::endl; // https://www.qrcode-monkey.com/fr/#text
        // Srv:123456789012345678901234567890;Usr:@123456789012345678901234567890
        // Affiche la clé public, un bouton pour obtenir la clé privé via le mdp, et une pp qui sera aussi dans la bdd du serveur de nœux (300px x 300px)
    }
    ImGui::SameLine();
    ImGui::Text("@username");
    ImGui::SameLine();
    if (ImGui::Button("Par", ImVec2(40, 40))) { // Bouton avec logique
        std::cout << "Afficher les paramêtres" << std::endl;
        // Selection du serveur de nœux principale
    }


    ImGui::End();


    // --- Zone info contact --
    ImGui::SetNextWindowPos(ImVec2(CONTACT_ZONE_X_SIZE, 0));
    ImGui::SetNextWindowSize(ImVec2(INFO_ZONE_X_SIZE, INFO_ZONE_Y_SIZE));
    ImGui::Begin("Info", NULL, ImGuiWindowFlags_NoDecoration);

    ImGui::Text("@%s", contacts[_actual_user_conv_index].username.c_str());

    ImGui::End();


    // --- Zone de Chat ---
    ImGui::SetNextWindowPos(ImVec2(LOGO_ZONE_X_SIZE, LOGO_ZONE_Y_SIZE));
    ImGui::SetNextWindowSize(ImVec2(CHAT_ZONE_X_SIZE, CHAT_ZONE_Y_SIZE));
    ImGui::Begin("Chat", NULL, ImGuiWindowFlags_NoDecoration);
    /*
    if (ImGui::Button("Test api /")) { // Bouton avec logique
        web_r.get((std::string)"http://127.0.0.1:5080");
        std::cout << web_r.get_web_answer() << std::endl;
    }
    */
    std::vector<message_arrage> msg_test = {
        {"Enzo", "Salut ! J'ai enfin regle les erreurs de compilation.", "2024-05-20", "14:35"},
        {"Enzo", "Mais c'était pas simple. Tu veux call  pour voir ?", "2024-05-20", "14:36"},
        {"Pierre", "Aller, mais dans 30 min", "2024-05-20", "14:35"},
        {"Pierre", "Je dois m'occuper de mon horrible chien", "2024-05-20", "14:35"},
        {"Pierre", "P.S. : Que je deteste", "2024-05-20", "14:35"},
        {"Pierre", "P.P.S. : Car je suis pas cool", "2024-05-20", "14:35"},
        {"Enzo", "OK tkt c'est pas grave je peux attendre", "2024-05-20", "14:35"},
        {"Enzo", "Enfant de Montessori", "2024-05-20", "14:35"},
    };

    static std::string last_message_username = "";
    last_message_username = "";
    ImGui::BeginChild("messages", ImVec2(0, 0), false, ImGuiWindowFlags_HorizontalScrollbar);
        ImGui::TextDisabled("Ceci est le début de l'historique de tes messages privés avec %s", contacts[_actual_user_conv_index].username.c_str());
        for(int i = 0; i < msg_test.size(); i++) {
            if(last_message_username == msg_test[i].username) {
                ImGui::Text(msg_test[i].msg.c_str());
                //* TODO: Si je passe sur le text
                /*
                ImGui::SameLine();
                ImGui::TextDisabled("%s", msg_test[i].hour.c_str());
                */
            }
            else {
                ImGui::Dummy(ImVec2(0.0f, 5.0f));
                message_struct(msg_test[i]);
            }
            last_message_username = msg_test[i].username;
        }
    
    ImGui::EndChild();

    ImGui::End();






    // --- Zone de Saisie (Fixe en bas) ---
    // Position : Même X que le chat, mais Y = hauteur totale moins la taille de la barre
    ImGui::SetNextWindowPos(ImVec2(LOGO_ZONE_X_SIZE, LOGO_ZONE_Y_SIZE + CHAT_ZONE_Y_SIZE));
    ImGui::SetNextWindowSize(ImVec2(CHAT_ZONE_X_SIZE, INPUT_ZONE_Y_SIZE));

    // On peut mettre une couleur de fond légèrement différente pour la barre de saisie
    ImGui::PushStyleColor(ImGuiCol_WindowBg, ImVec4(0.20f, 0.21f, 0.24f, 1.00f)); 
    ImGui::Begin("Input_Bar", NULL, ImGuiWindowFlags_NoDecoration);

    ImGui::SetCursorPosY(10); // Petit padding vertical interne
    ImGui::PushItemWidth(-10); // On laisse 10px de marge à droite
    
    if (ImGui::InputTextWithHint("##ChatInput", "Écrire dans #general...", _message_buffer, IM_ARRAYSIZE(_message_buffer), ImGuiInputTextFlags_EnterReturnsTrue)) {
        if (strlen(_message_buffer) > 0) {
            std::cout << "Envoi : " << _message_buffer << std::endl;
            memset(_message_buffer, 0, sizeof(_message_buffer));
            ImGui::SetKeyboardFocusHere(-1); 
        }
    }
    ImGui::PopItemWidth();

ImGui::End();
ImGui::PopStyleColor();

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

    ImGui::EndGroup();
}

void app::contact_struct(contact_arrage contact) {

    if (ImGui::Button(("@" + contact.username).c_str(), ImVec2(CONTACT_ZONE_X_SIZE - 16, 40))) { // Bouton avec logique
        switch_conv(contact.user_id);
    }

    /*
    ImGui::BeginGroup();
    
    // ImGui::Button(contact[0].c_str(), ImVec2(70, 20)); // Bouton icon

    if (ImGui::Button("Test api /")) { // Bouton avec logique
        web_r.get((std::string)"http://127.0.0.1:5080");
        std::cout << web_r.get_web_answer() << std::endl;
    }

    ImGui::Spacing();
    ImGui::Text("@%s",contact.username.c_str());
    ImGui::SameLine();
    ImGui::Text(contact.status ? "P2P active" : "P2P inactive");
    ImGui::SameLine();
    ImGui::Text(std::to_string(contact.user_id).c_str());
    
    ImGui::Spacing();
    ImGui::EndGroup();
    */
}

void app::switch_conv(const int& user_id) {
    if (user_id == _actual_user_conv_index) return;
    _actual_user_conv_index = user_id;
    std::cout << "Passage à la converssation de l'utilisateur n°" << user_id << std::endl;
}