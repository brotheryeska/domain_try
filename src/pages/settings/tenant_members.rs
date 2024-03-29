use yew::prelude::*;

pub struct SettingsTenantMembers {}

pub enum Msg {}

impl Component for SettingsTenantMembers {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        SettingsTenantMembers {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div
                    class="d-flex mb-4"
                >
                    <div>
                        {"Tenant Members have varying levels of access to the Auth0 dashboard based on their Roles, which are set when adding new members and can be edited any time. Learn more."}
                    </div>
                    <div
                        class="d-flex ms-3"
                    >
                        <select class="form-select d-inline-block me-2 w-auto" aria-label="Default select example">
                            <option selected=true>{"Members"}</option>
                            <option value="1">{"One"}</option>
                            <option value="2">{"Two"}</option>
                            <option value="3">{"Three"}</option>
                        </select>
                        <button
                            type="button"
                            class="btn btn-primary text-nowrap"
                            data-bs-toggle="modal" data-bs-target="#exampleModal"
                        >
                            <span>{"Add Member"}</span>
                        </button>
                    </div>
                </div>

                <table class="table">
                    <thead>
                    <tr>
                        <th scope="col">{"Names"}</th>
                        <th scope="col">{"Roles"}</th>
                        <th scope="col">{"MFA"}</th>
                    </tr>
                    </thead>
                    <tbody>
                    <tr>
                        <td>
                            <div
                                class="p-2 d-flex"
                            >
                                <div
                                    style="flex: 0 0 auto; width: 40px; height: 40px; background-color: rgb(100,100,100);"
                                    class="d-flex justify-content-center align-items-center rounded-circle me-3"
                                >
                                    <i class="bi bi-info-lg text-light"></i>
                                </div>

                                <div
                                    class="d-grid"
                                    style="min-width: 40px;"
                                >
                                    <span
                                        class="fw-bold"
                                        style="
                                            white-space: nowrap;
                                            text-overflow: ellipsis;
                                            overflow: hidden;
                                            font-size: 14px;
                                            text-decoration: none;
                                        "
                                    >
                                        {"ronaldo"}
                                    </span>
                                    <p
                                        class="mb-0 text-muted"
                                        style="
                                            white-space: nowrap;
                                            text-overflow: ellipsis;
                                            overflow: hidden;
                                            font-size: 14px;
                                        "
                                    >
                                        {"ronaldo@gmail.com (google-oauth2)"}
                                    </p>
                                </div>

                            </div>
                        </td>
                        <td>{"Admin"}</td>
                        <td>
                            <span
                                class="badge bg-danger fw-bolder"
                                style="text-transform: uppercase; letter-spacing: 1px; font-size: 10px;"
                            >{"disabled"}</span>
                        </td>
                    </tr>
                    </tbody>
                </table>


                // MODAL NEW MEMBER
                <div
                    class="modal fade"
                    id="exampleModal"
                    tabindex="-1"
                    aria-labelledby="exampleModalLabel"
                    aria-hidden="true"
                >
                    <div class="modal-dialog modal-dialog-scrollable">
                        <div class="modal-content">
                            <div class="modal-header">
                                <h5 class="modal-title" id="exampleModalLabel">{"Add New Tenant Member"}</h5>
                                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                            </div>
                            <div class="modal-body" style="font-size: 14px;">
                                <div
                                    class="mb-4"
                                >
                                    <label for="basic-url" class="form-label fw-bold">{"Email*"}</label>
                                    <div class="input-group mb-2">
                                        <input
                                            type="text"
                                            class="form-control"
                                            id="basic-url"
                                            aria-describedby="basic-addon3"
                                            placeholder="john@mail.com"
                                        />
                                    </div>
                                </div>
                                <div
                                    class="mb-4"
                                >
                                    <label for="basic-url" class="form-label fw-bold">{"Roles"}</label>
                                    <div class="alert alert-warning mb-5" role="alert">
                                        <i class="bi bi-exclamation-triangle me-2"></i>
                                        {"In order to take advantage of all available roles, please consider upgrading your Auth0 subscription"}
                                    </div>


                                    <div>

                                        <div class="form-check mb-3">
                                            <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"/>
                                            <label class="form-check-label d-grid" for="flexCheckDefault">
                                                <span
                                                    style="
                                                        font-size: 14px;
                                                        text-decoration: none;
                                                    "
                                                >
                                                    {"Admin"}
                                                </span>
                                                <p
                                                    class="mb-0 text-muted"
                                                    style="
                                                        font-size: 14px;
                                                    "
                                                >
                                                    {"Read and write access to all resources in the dashboard."}
                                                </p>
                                            </label>
                                        </div>

                                        <div class="form-check mb-3">
                                            <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"/>
                                            <label class="form-check-label d-grid" for="flexCheckDefault">
                                                <span
                                                    style="
                                                        font-size: 14px;
                                                        text-decoration: none;
                                                    "
                                                >
                                                    {"Editor - Specific Apps"}
                                                </span>
                                                <p
                                                    class="mb-0 text-muted"
                                                    style="
                                                        font-size: 14px;
                                                    "
                                                >
                                                    {"Read and write access to specific applications only."}
                                                </p>
                                            </label>
                                        </div>

                                        <div class="form-check mb-3">
                                            <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"/>
                                            <label class="form-check-label d-grid" for="flexCheckDefault">
                                                <span
                                                    style="
                                                        font-size: 14px;
                                                        text-decoration: none;
                                                    "
                                                >
                                                    {"Editor - Connections"}
                                                </span>
                                                <p
                                                    class="mb-0 text-muted"
                                                    style="
                                                        font-size: 14px;
                                                    "
                                                >
                                                    {"Read, write, and create access to all types of connections."}
                                                </p>
                                            </label>
                                        </div>

                                        <div class="form-check mb-3">
                                            <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"/>
                                            <label class="form-check-label d-grid" for="flexCheckDefault">
                                                <span
                                                    style="
                                                        font-size: 14px;
                                                        text-decoration: none;
                                                    "
                                                >
                                                    {"Editor - Users"}
                                                </span>
                                                <p
                                                    class="mb-0 text-muted"
                                                    style="
                                                        font-size: 14px;
                                                    "
                                                >
                                                    {"User Management operations (create, delete, block, unblock, reset MFA, reset password, update metadata, assign roles, etc.) and access to logs."}
                                                </p>
                                            </label>
                                        </div>

                                        <div class="form-check mb-3">
                                            <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"/>
                                            <label class="form-check-label d-grid" for="flexCheckDefault">
                                                <span
                                                    style="
                                                        font-size: 14px;
                                                        text-decoration: none;
                                                    "
                                                >
                                                    {"Viewer - Users"}
                                                </span>
                                                <p
                                                    class="mb-0 text-muted"
                                                    style="
                                                        font-size: 14px;
                                                    "
                                                >
                                                    {"Read-only access to users and logs."}
                                                </p>
                                            </label>
                                        </div>

                                        <div class="form-check mb-3">
                                            <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"/>
                                            <label class="form-check-label d-grid" for="flexCheckDefault">
                                                <span
                                                    style="
                                                        font-size: 14px;
                                                        text-decoration: none;
                                                    "
                                                >
                                                    {"Viewer - Config Settings"}
                                                </span>
                                                <p
                                                    class="mb-0 text-muted"
                                                    style="
                                                        font-size: 14px;
                                                    "
                                                >
                                                    {"Read-only access to all configuration settings (applications, APIs, rules, security settings, etc.), except for sensitive information such as secrets, billings, users, and logs."}
                                                </p>
                                            </label>
                                        </div>

                                    </div>

                                </div>
                            </div>
                            <div class="modal-footer">
                                <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Cancel"}</button>
                                <button type="button" class="btn btn-primary">{"Invite"}</button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
