use yew::prelude::*;
use yew_router::prelude::*;
// use yew_router::components::RouterAnchor;
use yew::services::{
    ConsoleService,
    storage::{ StorageService, Area },
};
use yew::format::{ Json };
// use yewdux::prelude::*;
use yewdux::prelude::WithDispatch;
use yewdux::dispatch::Dispatcher;
use yewtil::NeqAssign;
// use yew_router::switch::{Permissive};
use yew_router::route::Route;
use yew_router::service::RouteService;

use crate::store::reducer_account::{
    AppDispatch,
    DataAccountAction,
    // DataAccount,
};
use crate::types::{
    ResponseLogin,
};

use crate::pages::{

    outer::{
        login_page::LoginPage,
        register_page::RegisterPage,
        password_page::RequestPassPage,
    },
    
    home_page::HomePage,
    getting_started::GettingStarted,
    activity::Activity,

    applications::{
        applications::home::ApplicationHome,
        apis::{
            home::ApisHome,
            settings::ApisSettings,
        },
        sso::{
            home::SsoHome,
            create_sso::CreateSso,
        },
    },

    authentication::{
        database::{
            home::DatabaseHome,
            create_db::DbCreate,
            settings::DatabaseSettings,
        },
        social::{
            home::SocialHome,
            settings::SocialSettings,
            create::SocialCreate,
        },
        enterprise::{
            home::EnterpriseHome,
            google_apps::EnterpriseGoogle,
            google_apps_create::EnterpriseGoogleCreate,
        },
        passwordless::home::AuthPasswordLess,
    },

    settings::{
        home::SettingsHome,
    },

    management::{
        users::{
            home::UsersManagement,
            user_viewdetail::UserViewDetail,
        },
        roles::{
            // home::RolesManagement,
            role_created::RolesCreated,
            dropdown_viewdetail::ViewDetail,
        },
    },
};

use crate::components::{
    navtop::Navtop,
    sidebar::Sidebar,
};

use crate::types::LocalStorage;
use crate::types::LOCALSTORAGE_KEY;

#[derive(Switch, Clone)]
pub enum AppRoute {
    // MEMBER PAGES
    #[to = "/apis/settings"]
    ApisSettings,
    #[to = "/getting-started"]
    GettingStarted,
    #[to = "/{tenant_id}/apis"]
    ApisHome { tenant_id: String },
    #[to = "/activity"]
    Activity,
    #[to = "/applications"]
    DatabaseSettings,
    #[to = "/authentication/database/settings"]
    DbCreate,
    #[to = "/authentication/database/create"]
    DatabaseHome,
    #[to = "/authentication/database"]
    ApplicationHome,
    #[to = "/authentication/passwordless"]
    AuthPasswordless,
    #[to = "/sso/create-sso"]
    CreateSso,
    #[to = "/sso"]
    SsoHome,
    #[to = "/social/create"]
    SocialCreate,
    #[to = "/social/settings"]
    SocialSettings,
    #[to = "/social"]
    SocialHome,
    #[to = "/user-management/roles/settings"]
    ViewDetail,
    #[to = "/user-management/roles"]
    RolesCreated,
    #[to="/user-management/users/setting"]
    UserViewDetail,
    #[to = "/user-management/users"]
    UsersManagement,
    #[to = "/enterprise/google-app/create"]
    EnterpriseGoogleCreate,
    #[to = "/enterprise/google-app"]
    EnterpriseGoogle,
    #[to = "/enterprise"]
    EnterpriseHome,
    #[to = "/tenant"]
    SettingsHome,

    // NOT LOGGED IN PAGES
    #[to = "/login/password"]
    RequestPassPage,
    #[to = "/login"]
    LoginPage,
    #[to = "/register"]
    RegisterPage,
    #[to = "/"]
    Home,
}

pub struct App {
    dispatch: AppDispatch,
    // link: ComponentLink<Self>,
}

pub enum Msg {
    AutoLogin(ResponseLogin),
    SetIsAuth(bool),
}

impl Component for App {
    type Message = Msg;
    type Properties = AppDispatch;

    fn create(dispatch: Self::Properties, link: ComponentLink<Self>) -> Self {
        
        let storage = StorageService::new(Area::Local).expect("storage was disabled");

        // LOCALSTORAGE RESOURCE
        // https://github.com/yewstack/yew/issues/1287
        // GET LOCALSTORAGE
        // NEED BETTER WAY TO PARSE JSON DATA
        let localstorage_data = {
            if let Json(Ok(data)) = storage.restore(LOCALSTORAGE_KEY) {
                // ConsoleService::info("get localstorage");
                ConsoleService::info(&format!("{:?}", data));

                data
            } else {
                ConsoleService::info("token does not exist");
                LocalStorage {
                    username: None,
                    email: None,
                    token: None,
                }
            }
        };

        ConsoleService::info(&format!("{:?}", localstorage_data));

        // IF LOCALSTORAGE EXISTS
        // UPDATE REDUCER
        if let Some(_) = localstorage_data.username {
            let data_account = ResponseLogin {
                // username: Some(String::from(data.username.unwrap())),
                // email: Some(String::from(data.email.unwrap())),
                // token: Some(String::from(data.token.unwrap())),
                username: localstorage_data.username.unwrap(),
                email: localstorage_data.email.unwrap(),
                token: localstorage_data.token.unwrap(),
            };
            // dispatch.send(DataAccountAction::Update(data_account));
            link.send_message(Msg::AutoLogin(data_account));
        } else {
            link.send_message(Msg::SetIsAuth(false));
        }



        App {
            dispatch,
            // link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AutoLogin(user) => {
                ConsoleService::info("autologin");
                self.dispatch.send(DataAccountAction::Update(user));
                self.dispatch.send(DataAccountAction::SetIsAuth(false));
                true
            }
            Msg::SetIsAuth(state) => {
                ConsoleService::info("set is auth from app");
                self.dispatch.send(DataAccountAction::SetIsAuth(state));
                true
            }
        }
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        self.dispatch.neq_assign(dispatch)
    }

    fn view(&self) -> Html {

        // let acc_ref = &account;
        let acc = self.dispatch.state().clone();
        let is_auth = acc.is_auth;
        let is_logged_in = if acc.username == None {false} else {true};
        // let route_service = RouteService::new();
        let render = Router::render(move |switch: AppRoute| {
            let mut route_service = RouteService::new();
            match switch {
                // NOT LOGGED IN ROUTES
                AppRoute::Home => {
                    if is_logged_in {
                        route_service.set_route("/manage", ());
                        html! {<GettingStarted/>}
                    } else {
                        html! {<HomePage/>}
                    }
                },
                AppRoute::LoginPage => {
                    if is_logged_in {
                        route_service.set_route("/manage", ());
                        html! {<GettingStarted/>}
                    } else {
                        html! {<WithDispatch<LoginPage>/>}
                    }
                },
                AppRoute::RegisterPage => {
                    if is_logged_in {
                        route_service.set_route("/manage", ());
                        html! {<GettingStarted/>}
                    } else {
                        html!{<RegisterPage/>}
                    }
                },
                AppRoute::RequestPassPage => {
                    if is_logged_in {
                        route_service.set_route("/manage", ());
                        html! {<GettingStarted/>}
                    } else {
                        html!{<RequestPassPage/>}
                    }
                },






                // LOGGED IN ROUTES
                AppRoute::Activity => {
                    if is_logged_in {
                        html!{<Activity/>}
                    } else {
                        route_service.set_route("/", ());
                        html! {<HomePage/>}
                    }
                },
                AppRoute::GettingStarted => {
                    if is_logged_in {
                        html! {<GettingStarted/>}
                    } else {
                        route_service.set_route("/", ());
                        html! {<HomePage/>}
                    }
                },
                AppRoute::ApisHome{ tenant_id } => {
                    if is_logged_in {
                        html! {<ApisHome tenant_id=tenant_id />}
                    } else {
                        route_service.set_route("/", ());
                        html! {<HomePage/>}
                    }
                },
                AppRoute::ApisSettings => {
                    if is_logged_in {
                        html! {<ApisSettings/>}
                    } else {
                        route_service.set_route("/", ());
                        html! {<HomePage/>}
                    }
                },
                AppRoute::ApplicationHome => {
                    if is_logged_in {
                        html! {<ApplicationHome/>}
                    } else {
                        route_service.set_route("/", ());
                        html! {<HomePage/>}
                    }
                },
                AppRoute::AuthPasswordless => {
                    if is_logged_in {
                        html! {<AuthPasswordLess/>}
                    } else {
                        route_service.set_route("/", ());
                        html! {<HomePage/>}
                    }
                },
                AppRoute::SsoHome => {
                    if is_logged_in {
                        html! {<SsoHome/>}
                    } else {
                        route_service.set_route("/", ());
                        html! {<HomePage/>}
                    }
                },
                AppRoute::CreateSso => {
                    if is_logged_in {
                        html! {<CreateSso/>}
                    } else {
                        route_service.set_route("/", ());
                        html! {<HomePage/>}
                    }
                },
                AppRoute::SocialHome => {
                    if is_logged_in {
                        html! {<SocialHome/>}
                    } else {
                        route_service.set_route("/", ());
                        html! {<HomePage/>}
                    }
                },
                AppRoute::SocialSettings => {
                    if is_logged_in {
                        html! {<SocialSettings/>}
                    } else {
                        route_service.set_route("/", ());
                        html! {<HomePage/>}
                    }
                },
                AppRoute::SocialCreate => {
                    if is_logged_in {
                        html! {<SocialCreate/>}
                    } else {
                        route_service.set_route("/", ());
                        html! {<HomePage/>}
                    }
                },
                AppRoute::RolesCreated => {
                    if is_logged_in {
                        html! {<RolesCreated/>}
                    } else {
                        route_service.set_route("/", ());
                        html! {<HomePage/>}
                    }
                },
                AppRoute::UsersManagement => {
                    if is_logged_in {
                        html! {<UsersManagement/>}
                    } else {
                        route_service.set_route("/", ());
                        html! {<HomePage/>}
                    }
                },
                AppRoute::UserViewDetail => {
                    if is_logged_in {
                        html! {<UserViewDetail/>}
                    } else {
                        route_service.set_route("/", ());
                        html! {<HomePage/>}
                    }
                }
                AppRoute::EnterpriseHome => {
                    if is_logged_in {
                        html! {<EnterpriseHome/>}
                    } else {
                        route_service.set_route("/", ());
                        html! {<HomePage/>}
                    }
                },
                AppRoute::EnterpriseGoogle => {
                    if is_logged_in {
                        html! {<EnterpriseGoogle/>}
                    } else {
                        route_service.set_route("/", ());
                        html! {<HomePage/>}
                    }
                },
                AppRoute::EnterpriseGoogleCreate => {
                    if is_logged_in {
                        html! {<EnterpriseGoogleCreate/>}
                    } else {
                        route_service.set_route("/", ());
                        html! {<HomePage/>}
                    }
                },
                AppRoute::SettingsHome => {
                    if is_logged_in {
                        html! {<SettingsHome/>}
                    } else {
                        route_service.set_route("/", ());
                        html! {<HomePage/>}
                    }
                },
                AppRoute::ViewDetail => {
                    if is_logged_in {
                        html! {<ViewDetail/>}
                    } else {
                        route_service.set_route("/", ());
                        html! {<HomePage/>}
                    }
                },
                AppRoute::DatabaseHome => {
                    if is_logged_in {
                        html! {<DatabaseHome/>}
                    } else {
                        route_service.set_route("/", ());
                        html! {<HomePage/>}
                    }
                },
                AppRoute::DbCreate => {
                    if is_logged_in {
                        html! {<DbCreate/>}
                    } else {
                        route_service.set_route("/", ());
                        html! {<HomePage/>}
                    }
                },
                AppRoute::DatabaseSettings => {
                    if is_logged_in {
                        html! {<DatabaseSettings/>}
                    } else {
                        route_service.set_route("/", ());
                        html! {<HomePage/>}
                    }
                },






                // OTHER ROUTES
                _ => {
                    if is_logged_in {
                        route_service.set_route("/manage", ());
                        html! {<GettingStarted/>}
                    } else {
                        route_service.set_route("/", ());
                        html! {<HomePage/>}
                    }
                },
            }

            // if is_logged_in {
            //     match switch {
            //         AppRoute::Activity => html!{<Activity/>},
            //         AppRoute::GettingStarted => html! {<GettingStarted/>},
            //         AppRoute::ApisHome{ tenant_id } => html! {<ApisHome tenant_id=tenant_id />},
            //         AppRoute::ApisSettings => html! {<ApisSettings/>},
            //         AppRoute::ApplicationHome => html! {<ApplicationHome/>},
            //         AppRoute::AuthPasswordless => html! {<AuthPasswordLess/>},
            //         AppRoute::SsoHome => html! {<SsoHome/>},
            //         AppRoute::CreateSso => html! {<CreateSso/>},
            //         AppRoute::SocialHome => html! {<SocialHome/>},
            //         AppRoute::SocialSettings => html! {<SocialSettings/>},
            //         AppRoute::SocialCreate => html! {<SocialCreate/>},
            //         AppRoute::RolesCreated => html! {<RolesCreated/>},
            //         AppRoute::UsersManagement => html! {<UsersManagement/>},
            //         AppRoute::EnterpriseHome => html! {<EnterpriseHome/>},
            //         AppRoute::EnterpriseGoogle => html! {<EnterpriseGoogle/>},
            //         AppRoute::EnterpriseGoogleCreate => html! {<EnterpriseGoogleCreate/>},
            //         AppRoute::SettingsHome => html! {<SettingsHome/>},
            //         AppRoute::ViewDetail => html! {<ViewDetail/>},
            //         AppRoute::DatabaseHome => html! {<DatabaseHome/>},
            //         AppRoute::DbCreate => html! {<DbCreate/>},
            //         AppRoute::DatabaseSettings => html! {<DatabaseSettings/>},
            //         _ => {
            //             // ConsoleService::info("SET ROUTE TO MANAGE");
            //             route_service.set_route("/manage", ());
            //             html! {<GettingStarted/>}
            //         },
            //     }
            // } else {
            //     match switch {
            //         AppRoute::Home => {
            //             // ConsoleService::info("ROUTE HOMEPAGE");
            //             html! {<HomePage/>}
            //         },
            //         AppRoute::LoginPage => html! {<WithDispatch<LoginPage>/>},
            //         AppRoute::RegisterPage => html!{<RegisterPage/>},
            //         AppRoute::RequestPassPage => html!{<RequestPassPage/>},
            //         _ => {
            //             // ConsoleService::info("SET ROUTE /");
            //             route_service.set_route("/", ());
            //             html! {<HomePage/>}
            //         },
            //     }
            // }




            // match switch {
            //     AppRoute::GettingStarted => html! {<GettingStarted/>},
            //     AppRoute::ApisHome if !is_logged_in => {
            //         ConsoleService::info("redirect");
            //         route_service.set_route("/", ());
            //         html! {<HomePage/>}
            //     },
            //     AppRoute::ApisHome => html! {<ApisHome/>},
            //     AppRoute::Settings => html! {<Settings/>},
            //     AppRoute::ApplicationHome => html! {<ApplicationHome/>},
            //     AppRoute::Home if !is_logged_in => html!{<HomePage/>}, 
            //     AppRoute::Home => {
            //         route_service.set_route("/manage", ());
            //         html! {<GettingStarted/>}
            //     },
            //     // html! {<HomePage/>},
            //     AppRoute::LoginPage if !is_logged_in => {html! {<WithDispatch<LoginPage>/>}},
            //     AppRoute::LoginPage => {
            //         ConsoleService::info("redirect");
            //         // self.route_service.set_route("/manage", ());
            //         route_service.set_route("/manage", ());
            //         html! {<GettingStarted/>}
            //     },
            //     AppRoute::RegisterPage => html!{<RegisterPage/>},
            //     AppRoute::RequestPassPage => html!{<RequestPassPage/>},
            //     // _ => html! {
            //     //     <GettingStarted/>
            //     // },
            // }
        });

        let account = self.dispatch.state().clone();

        if is_logged_in && !is_auth {
            html! {
                <>
                    <WithDispatch<Navtop>/>
                    
                    <div
                        class="container-fluid"
                    >
                        <div
                            class="row flex-nowrap"
                        >
                            <WithDispatch<Sidebar>/>
                            <div 
                                class="col"
                            >
                                <Router<AppRoute, ()>
                                    render=render
                                    // https://github.com/yewstack/yew_router/blob/master/examples/router_component/src/main.rs#L88
                                    redirect = Router::redirect(|route: Route| {
                                        ConsoleService::info(&route.route);
                                        AppRoute::LoginPage
                                        // Route::PageNotFound(Permissive(Some(route.route)))
                                    })
                                />
                            </div>
                        </div>
                        
                    </div>
                    // <TestingFetch/>
                    // <p></p>
                    // <p>{"Reducer"}</p>
                    // <WithDispatch<ReducerGlobal>/>
                    // <WithDispatch<ReducerAccountView>/>
                </>
            }
        } else if !is_auth {
            html! {
                <>
                    <main>
                        <Router<AppRoute, ()>
                            render=render
                            redirect = Router::redirect(|route: Route| {
                                ConsoleService::info(&route.route);
                                AppRoute::LoginPage
                            })
                        />
                    </main>
                </>
            }
        } else {
            html! {
                <div>
                    {"LOADING..."}
                </div>
            }
        }

    }
}

// impl App {
//     fn navtop(&self, account: DataAccount) -> Html {

//         if account.name == None {
//             html! {
//                 <>
//                     <HomePage/>
//                 </>
//             }
//         } else {
//             html! {
//                 <Navtop/>
//             }
//         }
//     }
// }
