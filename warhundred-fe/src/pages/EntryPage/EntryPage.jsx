import {Tabs, TabList, TabPanel, Tab} from "react-tabs";
import {RegisterForm} from "./components/RegisterForm.jsx";
import {LoginForm} from "./components/LoginForm.jsx";
import "./EntryPage.css";

export const EntryPage = () => {
  return (
      <>
        <main className="entry-page__main">
          <h2>Welcome to War Hundred</h2>
          <Tabs>
            <TabList className="entry-page__tab-list">
              <Tab className="entry-page__list-item">
                <button className="entry-page__tab-list-btn">Register</button>
              </Tab>
              <Tab className="entry-page__list-item">
                <button className="entry-page__tab-list-btn">Login</button>
              </Tab>
            </TabList>

            <TabPanel>
              <RegisterForm></RegisterForm>
            </TabPanel>
            <TabPanel>
              <LoginForm></LoginForm>
            </TabPanel>
          </Tabs>
        </main>
        <footer>All rights reserved (c)</footer>
      </>
  )
}

export default EntryPage;