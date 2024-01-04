import {Tabs, TabList, TabPanel, Tab} from "react-tabs";
import {RegisterForm} from "./components/RegisterForm.jsx";
import {LoginForm} from "./components/LoginForm.jsx";

export const MainPage = () => {
    return (
        <>
          <h2>Welcome to War Hundred</h2>
          <Tabs className="main-page__main">
            <TabList className="main-page__tab-list">
              <Tab className="main-page__list-item">
                <button className="main-page__tab-list-btn">Register</button>
              </Tab>
              <Tab className="main-page__list-item">
                <button className="main-page__tab-list-btn">Login</button>
              </Tab>
            </TabList>

            <TabPanel>
              <RegisterForm></RegisterForm>
            </TabPanel>
            <TabPanel>
              <LoginForm></LoginForm>
            </TabPanel>
          </Tabs>
        </>
    )
}