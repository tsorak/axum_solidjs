import { A } from "@solidjs/router";
import logo from "../logo.svg";

import styles from "./Layout/Layout.module.sass";

export default function Layout(props) {
  return (
    <div class={styles.main_layout}>
      <Header />
      {props.children}
      <Footer />
    </div>
  );
}

function Header() {
  return (
    <header class={styles.header}>
      <nav>
        <NavItems />
      </nav>
    </header>
  );
}

function Footer() {
  return (
    <footer class={styles.footer}>
      <nav>
        <NavItems />
      </nav>
      <div>
        <img src={logo} width="32" alt="logo" />
      </div>
      <div>
        <p>Don't call me :)</p>
      </div>
    </footer>
  );
}

function NavItems() {
  return (
    <>
      <A href="/" class={styles.nav_link}>
        Home
      </A>
      <A href="/motd" class={styles.nav_link}>
        Message Of The Day
      </A>
    </>
  );
}
