import { FaGithub } from "react-icons/fa";
import "./styles/footer.css"

export function Footer() {
  return (
    <section className="about-me">
      <h3 className="about-me__title">
        <span className="fade">A&nbsp;</span>

        <span className="color-gb-yellow">Rust</span>

        <span className="fade">&nbsp;Com</span>

        <span className="color-gb-yellow">piler </span>

        <span className="fade"> Written in Rust</span>
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
