-- Your SQL goes here
CREATE TABLE `b0enr5m529ydki4frstg`.`Estaciones` (
  `id_Estacion` INT NOT NULL AUTO_INCREMENT,
  `alias` VARCHAR(45) NOT NULL,
  `nombre_estacion_tiempo_presente` VARCHAR(45) NULL,
  `nombre_estacion_dato_horario` VARCHAR(45) NULL,
  `nombre_estacion_pronostico` VARCHAR(45) NULL,
  PRIMARY KEY (`id_Estacion`));
