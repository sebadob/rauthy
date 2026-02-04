// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="intro.html"><strong aria-hidden="true">1.</strong> Introduction</a></li><li class="chapter-item expanded "><a href="getting_started/main.html"><strong aria-hidden="true">2.</strong> Getting Started</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="getting_started/docker.html"><strong aria-hidden="true">2.1.</strong> Docker</a></li><li class="chapter-item expanded "><a href="getting_started/k8s.html"><strong aria-hidden="true">2.2.</strong> Kubernetes</a></li><li class="chapter-item expanded "><a href="getting_started/first_start.html"><strong aria-hidden="true">2.3.</strong> First Start</a></li><li class="chapter-item expanded "><a href="getting_started/shutdown.html"><strong aria-hidden="true">2.4.</strong> Shutdown</a></li></ol></li><li class="chapter-item expanded "><a href="config/production_config.html"><strong aria-hidden="true">3.</strong> Production Config</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="config/config_minimal.html"><strong aria-hidden="true">3.1.</strong> Minimal Production Config</a></li><li class="chapter-item expanded "><a href="config/argon2.html"><strong aria-hidden="true">3.2.</strong> Password Hashing</a></li><li class="chapter-item expanded "><a href="config/passkeys.html"><strong aria-hidden="true">3.3.</strong> Passkeys</a></li><li class="chapter-item expanded "><a href="config/encryption.html"><strong aria-hidden="true">3.4.</strong> Encryption</a></li><li class="chapter-item expanded "><a href="config/logging.html"><strong aria-hidden="true">3.5.</strong> Logging and Auditing</a></li><li class="chapter-item expanded "><a href="config/backup.html"><strong aria-hidden="true">3.6.</strong> Backups</a></li><li class="chapter-item expanded "><a href="config/tls.html"><strong aria-hidden="true">3.7.</strong> TLS</a></li><li class="chapter-item expanded "><a href="config/sessions.html"><strong aria-hidden="true">3.8.</strong> Sessions</a></li><li class="chapter-item expanded "><a href="config/user_reg.html"><strong aria-hidden="true">3.9.</strong> User Registration</a></li><li class="chapter-item expanded "><a href="config/ha.html"><strong aria-hidden="true">3.10.</strong> High Availability</a></li><li class="chapter-item expanded "><a href="config/bootstrap.html"><strong aria-hidden="true">3.11.</strong> Bootstrapping</a></li><li class="chapter-item expanded "><a href="config/db_migration.html"><strong aria-hidden="true">3.12.</strong> Database Migrations</a></li><li class="chapter-item expanded "><a href="config/unix_socket.html"><strong aria-hidden="true">3.13.</strong> UNIX Domain Sockets</a></li><li class="chapter-item expanded "><a href="config/tuning.html"><strong aria-hidden="true">3.14.</strong> Tuning</a></li></ol></li><li class="chapter-item expanded "><a href="auth_providers/index.html"><strong aria-hidden="true">4.</strong> Authentication Providers</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="auth_providers/github.html"><strong aria-hidden="true">4.1.</strong> GitHub</a></li></ol></li><li class="chapter-item expanded "><a href="work/index.html"><strong aria-hidden="true">5.</strong> Working with Rauthy</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="work/api_keys.html"><strong aria-hidden="true">5.1.</strong> API Keys</a></li><li class="chapter-item expanded "><a href="work/custom_scopes_attributes.html"><strong aria-hidden="true">5.2.</strong> Custom Scopes and Attributes</a></li><li class="chapter-item expanded "><a href="work/ephemeral_clients.html"><strong aria-hidden="true">5.3.</strong> Ephemeral Clients</a></li><li class="chapter-item expanded "><a href="work/email_templates.html"><strong aria-hidden="true">5.4.</strong> E-Mail Templates</a></li><li class="chapter-item expanded "><a href="work/ip_blacklist.html"><strong aria-hidden="true">5.5.</strong> IP Blacklisting</a></li><li class="chapter-item expanded "><a href="work/jwks.html"><strong aria-hidden="true">5.6.</strong> JSON Web Keys</a></li><li class="chapter-item expanded "><a href="work/i18n.html"><strong aria-hidden="true">5.7.</strong> I18n</a></li><li class="chapter-item expanded "><a href="work/logout.html"><strong aria-hidden="true">5.8.</strong> User Logout</a></li><li class="chapter-item expanded "><a href="work/scim.html"><strong aria-hidden="true">5.9.</strong> SCIM</a></li><li class="chapter-item expanded "><a href="work/forward_auth.html"><strong aria-hidden="true">5.10.</strong> Forward Authentication</a></li><li class="chapter-item expanded "><a href="work/pam.html"><strong aria-hidden="true">5.11.</strong> PAM</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="work/pam_groups.html"><strong aria-hidden="true">5.11.1.</strong> Groups</a></li><li class="chapter-item expanded "><a href="work/pam_hosts.html"><strong aria-hidden="true">5.11.2.</strong> Hosts</a></li><li class="chapter-item expanded "><a href="work/pam_users.html"><strong aria-hidden="true">5.11.3.</strong> Users</a></li><li class="chapter-item expanded "><a href="work/pam_nss_modules.html"><strong aria-hidden="true">5.11.4.</strong> Modules</a></li></ol></li></ol></li><li class="chapter-item expanded "><a href="config/config.html"><strong aria-hidden="true">6.</strong> Reference Config</a></li><li class="chapter-item expanded "><a href="swagger.html"><strong aria-hidden="true">7.</strong> Swagger UI</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
