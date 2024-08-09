resource "kubernetes_persistent_volume" "mysql_pv" {
  metadata {
    name = "mysql-pv"
  }

  spec {
    access_modes       = ["ReadWriteOnce"]
    storage_class_name = "manual"

    capacity = {
      storage = "1Gi"
    }

    persistent_volume_source {
      host_path {
        path = "/mnt/data/mysql"
      }
    }
  }

}

resource "kubernetes_persistent_volume_claim" "mysql_pvc" {
  metadata {
    name      = "mysql-pvc"
    namespace = kubernetes_namespace.app.metadata[0].name
  }

  spec {
    access_modes       = ["ReadWriteOnce"]
    storage_class_name = "manual"

    resources {
      requests = {
        storage = "500Mi"
      }
    }
  }
}
