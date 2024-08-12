variable "container_ports" {
  type = map(number)
  default = {
    app = 80
    db  = 3306
  }
}

variable "database_storage_capacity" {
  type    = string
  default = "500Mi"
}
