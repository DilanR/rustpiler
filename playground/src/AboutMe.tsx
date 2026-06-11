import { FaGithub } from "react-icons/fa";
import "./styles/about_me.css"

export function AboutMe() {
  return (
    <section className="about-me">
      <h3 className="about-me__title">
        A Rust Compiler Written in Rust
      </h3>

      <a
        href="https://github.com/DilanR/rustpiler"
        target="_blank"
        rel="noopener noreferrer"
        className="about-me__github"
        aria-label="GitHub Profile"
      >
        <FaGithub size={24} />
      </a>
    </section>
  );
}
