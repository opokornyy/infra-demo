resource "kubernetes_namespace" "app" {
  metadata {
    name = "app"
  }
}

resource "kubernetes_secret" "db_password" {
  metadata {
    name      = "app-secret"
    namespace = kubernetes_namespace.app.metadata[0].name
  }

  data = {
    // TODO: Load from environment variables
    TLS_CERT            = base64encode(file("../tls.crt"))
    TLS_PRIVATE_KEY     = base64encode(file("../tls.key"))
    DATABASE_URL        = base64encode("mysql://root:root@db:3306/my_database")
    MYSQL_PASSWORD      = base64encode("root")
    MYSQL_ROOT_PASSWORD = base64encode("root")
  }
}
