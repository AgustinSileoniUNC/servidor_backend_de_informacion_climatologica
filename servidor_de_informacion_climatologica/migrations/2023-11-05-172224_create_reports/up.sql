CREATE TABLE `b0enr5m529ydki4frstg`.`REPORTES_DATO_HORARIO` (
  `id_reporte` INT NOT NULL AUTO_INCREMENT,
  `estacion` VARCHAR(45) NOT NULL,
  `fecha` VARCHAR(45) NOT NULL,
  `hora` VARCHAR(45) NOT NULL,
  `temperatura` VARCHAR(45) NULL,
  `humedad_relativa` VARCHAR(45) NULL,
  `presion_superficie` VARCHAR(45) NULL,
  `viento_direccion` VARCHAR(45) NULL,
  `viento_intensidad` VARCHAR(45) NULL,
  PRIMARY KEY (`id_reporte`));
